use anchor_lang::prelude::*;
use crate::state::farmer::{Farmer, FarmerStatus};
use crate::error::VeriFarmError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateFarmerProfileArgs {
    pub phone: Option<String>,
    pub location_lat: Option<i64>,
    pub location_lng: Option<i64>,
    /// Only a program admin can change status
    pub new_status: Option<FarmerStatus>,
}

#[derive(Accounts)]
pub struct UpdateFarmerProfile<'info> {
    #[account(
        mut,
        seeds = [Farmer::SEED, authority.key().as_ref()],
        bump = farmer.bump,
        has_one = authority,
    )]
    pub farmer: Account<'info, Farmer>,

    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateFarmerProfile>, args: UpdateFarmerProfileArgs) -> Result<()> {
    let farmer = &mut ctx.accounts.farmer;

    if let Some(phone) = args.phone {
        farmer.phone = phone;
    }
    if let Some(lat) = args.location_lat {
        farmer.location_lat = lat;
    }
    if let Some(lng) = args.location_lng {
        farmer.location_lng = lng;
    }
    // Status changes are gated — add admin check before enabling
    if args.new_status.is_some() {
        return err!(VeriFarmError::UnauthorizedOracle); // placeholder: wire up admin PDA
    }

    Ok(())
}
