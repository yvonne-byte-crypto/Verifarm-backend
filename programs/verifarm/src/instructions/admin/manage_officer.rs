use anchor_lang::prelude::*;
use crate::state::admin::{AdminConfig, LoanOfficerEntry};
use crate::error::VeriFarmError;

// ── Register Loan Officer ─────────────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(officer: Pubkey)]
pub struct RegisterOfficer<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + LoanOfficerEntry::INIT_SPACE,
        seeds = [LoanOfficerEntry::SEED, officer.as_ref()],
        bump,
    )]
    pub officer_entry: Account<'info, LoanOfficerEntry>,

    #[account(
        seeds = [AdminConfig::SEED],
        bump = admin_config.bump,
        has_one = admin @ VeriFarmError::UnauthorizedOracle,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn register_officer_handler(ctx: Context<RegisterOfficer>, officer: Pubkey) -> Result<()> {
    let entry = &mut ctx.accounts.officer_entry;
    entry.officer = officer;
    entry.added_by = ctx.accounts.admin.key();
    entry.active = true;
    entry.bump = ctx.bumps.officer_entry;
    entry.added_at = Clock::get()?.unix_timestamp;

    emit!(OfficerRegistered { officer, added_by: ctx.accounts.admin.key() });
    Ok(())
}

// ── Revoke Loan Officer ───────────────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(officer: Pubkey)]
pub struct RevokeOfficer<'info> {
    #[account(
        mut,
        seeds = [LoanOfficerEntry::SEED, officer.as_ref()],
        bump = officer_entry.bump,
    )]
    pub officer_entry: Account<'info, LoanOfficerEntry>,

    #[account(
        seeds = [AdminConfig::SEED],
        bump = admin_config.bump,
        has_one = admin @ VeriFarmError::UnauthorizedOracle,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    pub admin: Signer<'info>,
}

pub fn revoke_officer_handler(ctx: Context<RevokeOfficer>, _officer: Pubkey) -> Result<()> {
    ctx.accounts.officer_entry.active = false;
    emit!(OfficerRevoked { officer: ctx.accounts.officer_entry.officer });
    Ok(())
}

#[event]
pub struct OfficerRegistered { pub officer: Pubkey, pub added_by: Pubkey }

#[event]
pub struct OfficerRevoked { pub officer: Pubkey }
