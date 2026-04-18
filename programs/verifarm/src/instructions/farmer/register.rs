use anchor_lang::prelude::*;
use crate::state::farmer::{Farmer, FarmerStatus};
use crate::error::VeriFarmError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterFarmerArgs {
    pub national_id_hash: [u8; 32],
    pub full_name: String,
    pub phone: String,
    pub location_lat: i64,
    pub location_lng: i64,
}

#[derive(Accounts)]
#[instruction(args: RegisterFarmerArgs)]
pub struct RegisterFarmer<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Farmer::INIT_SPACE,
        seeds = [Farmer::SEED, authority.key().as_ref()],
        bump,
    )]
    pub farmer: Account<'info, Farmer>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterFarmer>, args: RegisterFarmerArgs) -> Result<()> {
    require!(args.full_name.len() <= 64, VeriFarmError::NameTooLong);

    let farmer_key = ctx.accounts.farmer.key();
    let authority_key = ctx.accounts.authority.key();
    let farmer = &mut ctx.accounts.farmer;
    farmer.authority = authority_key;
    farmer.national_id_hash = args.national_id_hash;
    farmer.full_name = args.full_name;
    farmer.phone = args.phone;
    farmer.location_lat = args.location_lat;
    farmer.location_lng = args.location_lng;
    farmer.status = FarmerStatus::Pending;
    farmer.loan_count = 0;
    farmer.on_time_repayments = 0;
    farmer.latest_risk_score = 0;
    farmer.bump = ctx.bumps.farmer;
    farmer.registered_at = Clock::get()?.unix_timestamp;

    emit!(FarmerRegistered {
        farmer: farmer_key,
        authority: authority_key,
        registered_at: farmer.registered_at,
    });

    Ok(())
}

#[event]
pub struct FarmerRegistered {
    pub farmer: Pubkey,
    pub authority: Pubkey,
    pub registered_at: i64,
}
