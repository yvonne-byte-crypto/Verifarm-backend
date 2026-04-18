use anchor_lang::prelude::*;
use crate::state::admin::{AdminConfig, LoanOfficerEntry};
use crate::state::farmer::Farmer;
use crate::state::farmer::FarmerStatus;
use crate::error::VeriFarmError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateFarmerStatusArgs {
    pub new_status: FarmerStatus,
    pub farmer_authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: UpdateFarmerStatusArgs)]
pub struct UpdateFarmerStatus<'info> {
    #[account(
        mut,
        seeds = [Farmer::SEED, args.farmer_authority.as_ref()],
        bump = farmer.bump,
    )]
    pub farmer: Account<'info, Farmer>,

    /// Officer entry must be active
    #[account(
        seeds = [LoanOfficerEntry::SEED, officer.key().as_ref()],
        bump = officer_entry.bump,
        constraint = officer_entry.active @ VeriFarmError::UnauthorizedOracle,
    )]
    pub officer_entry: Account<'info, LoanOfficerEntry>,

    #[account(
        seeds = [AdminConfig::SEED],
        bump = admin_config.bump,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    /// Either admin OR active loan officer can update status
    pub officer: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateFarmerStatus>, args: UpdateFarmerStatusArgs) -> Result<()> {
    let is_admin = ctx.accounts.officer.key() == ctx.accounts.admin_config.admin;
    let is_active_officer = ctx.accounts.officer_entry.officer == ctx.accounts.officer.key()
        && ctx.accounts.officer_entry.active;

    require!(
        is_admin || is_active_officer,
        VeriFarmError::UnauthorizedOracle
    );

    ctx.accounts.farmer.status = args.new_status.clone();

    emit!(FarmerStatusUpdated {
        farmer: ctx.accounts.farmer.key(),
        updated_by: ctx.accounts.officer.key(),
        new_status: args.new_status,
    });

    Ok(())
}

#[event]
pub struct FarmerStatusUpdated {
    pub farmer: Pubkey,
    pub updated_by: Pubkey,
    pub new_status: FarmerStatus,
}
