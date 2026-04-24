use anchor_lang::prelude::*;
use crate::state::agent::{AgentStake, AgentStatus};
use crate::error::VeriFarmError;

#[derive(Accounts)]
pub struct WithdrawStake<'info> {
    #[account(
        mut,
        seeds = [AgentStake::SEED, agent.key().as_ref()],
        bump = agent_stake.bump,
        constraint = agent_stake.agent == agent.key() @ VeriFarmError::Unauthorized,
        close = agent,
    )]
    pub agent_stake: Account<'info, AgentStake>,

    #[account(mut)]
    pub agent: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawStake>) -> Result<()> {
    require!(
        ctx.accounts.agent_stake.status == AgentStatus::Active,
        VeriFarmError::AgentSuspended
    );
    require!(
        ctx.accounts.agent_stake.active_verifications == 0,
        VeriFarmError::ActiveVerificationsPending
    );

    // `close = agent` in the account constraint transfers all lamports
    // (rent + staked SOL) to the agent and zeroes the account data.
    emit!(StakeWithdrawn {
        agent:            ctx.accounts.agent.key(),
        returned_lamports: ctx.accounts.agent_stake.staked_lamports,
        withdrawn_at:     Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct StakeWithdrawn {
    pub agent: Pubkey,
    pub returned_lamports: u64,
    pub withdrawn_at: i64,
}
