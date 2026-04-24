use anchor_lang::prelude::*;
use crate::state::agent::{AgentStake, AgentStatus, VerificationRecord, VerificationStatus, DISPUTE_WINDOW_SECS};
use crate::state::farmer::Farmer;
use crate::error::VeriFarmError;

#[derive(Accounts)]
pub struct SubmitVerification<'info> {
    #[account(
        init,
        payer = agent,
        space = 8 + VerificationRecord::INIT_SPACE,
        seeds = [
            VerificationRecord::SEED,
            agent.key().as_ref(),
            farmer.key().as_ref(),
        ],
        bump,
    )]
    pub verification_record: Account<'info, VerificationRecord>,

    #[account(
        mut,
        seeds = [AgentStake::SEED, agent.key().as_ref()],
        bump = agent_stake.bump,
        constraint = agent_stake.agent == agent.key() @ VeriFarmError::AgentNotRegistered,
    )]
    pub agent_stake: Account<'info, AgentStake>,

    pub farmer: Account<'info, Farmer>,

    #[account(mut)]
    pub agent: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SubmitVerification>) -> Result<()> {
    require!(
        ctx.accounts.agent_stake.status == AgentStatus::Active,
        VeriFarmError::AgentSuspended
    );

    let now        = Clock::get()?.unix_timestamp;
    let record_key = ctx.accounts.verification_record.key();

    let record = &mut ctx.accounts.verification_record;
    record.agent                = ctx.accounts.agent.key();
    record.farmer               = ctx.accounts.farmer.key();
    record.submitted_at         = now;
    record.dispute_window_ends  = now + DISPUTE_WINDOW_SECS;
    record.status               = VerificationStatus::Pending;
    record.disputed_by          = None;
    record.disputed_at          = None;
    record.bump                 = ctx.bumps.verification_record;

    ctx.accounts.agent_stake.active_verifications =
        ctx.accounts.agent_stake.active_verifications.saturating_add(1);

    let dispute_window_ends = record.dispute_window_ends;
    emit!(VerificationSubmitted {
        agent:                ctx.accounts.agent.key(),
        farmer:               ctx.accounts.farmer.key(),
        verification_record:  record_key,
        dispute_window_ends,
        submitted_at:         now,
    });

    Ok(())
}

#[event]
pub struct VerificationSubmitted {
    pub agent: Pubkey,
    pub farmer: Pubkey,
    pub verification_record: Pubkey,
    pub dispute_window_ends: i64,
    pub submitted_at: i64,
}
