use anchor_lang::prelude::*;
use crate::state::admin::AdminConfig;
use crate::state::agent::{AgentStake, AgentStatus, VerificationRecord, VerificationStatus};
use crate::error::VeriFarmError;

#[derive(Accounts)]
pub struct DisputeVerification<'info> {
    #[account(
        mut,
        seeds = [VerificationRecord::SEED, verification_record.agent.as_ref(), verification_record.farmer.as_ref()],
        bump = verification_record.bump,
    )]
    pub verification_record: Account<'info, VerificationRecord>,

    #[account(
        seeds = [AdminConfig::SEED],
        bump = admin_config.bump,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    /// Disputer must be either the admin OR a separately registered active agent.
    /// We validate this in the handler rather than in constraints so we can give
    /// a clear error message.
    #[account(mut)]
    pub disputer: Signer<'info>,

    /// Optional: disputer's own agent_stake (required when disputer is not admin).
    /// Pass the admin_config account again in place of this if disputer is admin.
    pub disputer_stake: Option<Account<'info, AgentStake>>,
}

pub fn handler(ctx: Context<DisputeVerification>) -> Result<()> {
    let now   = Clock::get()?.unix_timestamp;
    let admin = ctx.accounts.admin_config.admin;
    let disputer_key = ctx.accounts.disputer.key();

    // Authorisation: admin OR active registered agent
    let is_admin = disputer_key == admin;
    let is_active_agent = ctx.accounts.disputer_stake
        .as_ref()
        .map(|s| s.agent == disputer_key && s.status == AgentStatus::Active)
        .unwrap_or(false);

    require!(is_admin || is_active_agent, VeriFarmError::Unauthorized);

    let record = &mut ctx.accounts.verification_record;

    require!(
        record.status == VerificationStatus::Pending,
        VeriFarmError::VerificationNotPending
    );
    require!(
        now < record.dispute_window_ends,
        VeriFarmError::DisputeWindowExpired
    );

    record.status      = VerificationStatus::Disputed;
    record.disputed_by = Some(disputer_key);
    record.disputed_at = Some(now);

    emit!(VerificationDisputed {
        verification_record: ctx.accounts.verification_record.key(),
        disputer:            disputer_key,
        disputed_at:         now,
    });

    Ok(())
}

#[event]
pub struct VerificationDisputed {
    pub verification_record: Pubkey,
    pub disputer: Pubkey,
    pub disputed_at: i64,
}
