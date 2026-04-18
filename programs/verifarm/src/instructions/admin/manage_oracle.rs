use anchor_lang::prelude::*;
use crate::state::admin::{AdminConfig, OracleEntry};
use crate::error::VeriFarmError;

// ── Register Oracle ───────────────────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(oracle: Pubkey)]
pub struct RegisterOracle<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + OracleEntry::INIT_SPACE,
        seeds = [OracleEntry::SEED, oracle.as_ref()],
        bump,
    )]
    pub oracle_entry: Account<'info, OracleEntry>,

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

pub fn register_oracle_handler(ctx: Context<RegisterOracle>, oracle: Pubkey) -> Result<()> {
    let entry = &mut ctx.accounts.oracle_entry;
    entry.oracle = oracle;
    entry.added_by = ctx.accounts.admin.key();
    entry.active = true;
    entry.bump = ctx.bumps.oracle_entry;
    entry.added_at = Clock::get()?.unix_timestamp;

    emit!(OracleRegistered { oracle, added_by: ctx.accounts.admin.key() });
    Ok(())
}

// ── Revoke Oracle ─────────────────────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(oracle: Pubkey)]
pub struct RevokeOracle<'info> {
    #[account(
        mut,
        seeds = [OracleEntry::SEED, oracle.as_ref()],
        bump = oracle_entry.bump,
    )]
    pub oracle_entry: Account<'info, OracleEntry>,

    #[account(
        seeds = [AdminConfig::SEED],
        bump = admin_config.bump,
        has_one = admin @ VeriFarmError::UnauthorizedOracle,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    pub admin: Signer<'info>,
}

pub fn revoke_oracle_handler(ctx: Context<RevokeOracle>, _oracle: Pubkey) -> Result<()> {
    ctx.accounts.oracle_entry.active = false;
    emit!(OracleRevoked { oracle: ctx.accounts.oracle_entry.oracle });
    Ok(())
}

#[event]
pub struct OracleRegistered { pub oracle: Pubkey, pub added_by: Pubkey }

#[event]
pub struct OracleRevoked { pub oracle: Pubkey }
