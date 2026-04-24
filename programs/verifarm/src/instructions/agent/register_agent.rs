use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::agent::{AgentStake, AgentStatus, MIN_STAKE_LAMPORTS};
use crate::error::VeriFarmError;

#[derive(Accounts)]
pub struct RegisterAgent<'info> {
    #[account(
        init,
        payer = agent,
        space = 8 + AgentStake::INIT_SPACE,
        seeds = [AgentStake::SEED, agent.key().as_ref()],
        bump,
    )]
    pub agent_stake: Account<'info, AgentStake>,

    #[account(mut)]
    pub agent: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAgent>, stake_lamports: u64) -> Result<()> {
    require!(stake_lamports >= MIN_STAKE_LAMPORTS, VeriFarmError::InsufficientStake);

    // Transfer stake into the agent_stake PDA (on top of the rent already paid by init)
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.agent.to_account_info(),
                to:   ctx.accounts.agent_stake.to_account_info(),
            },
        ),
        stake_lamports,
    )?;

    let stake = &mut ctx.accounts.agent_stake;
    stake.agent               = ctx.accounts.agent.key();
    stake.staked_lamports     = stake_lamports;
    stake.active_verifications = 0;
    stake.status              = AgentStatus::Active;
    stake.bump                = ctx.bumps.agent_stake;
    stake.registered_at       = Clock::get()?.unix_timestamp;

    emit!(AgentRegistered {
        agent:             stake.agent,
        staked_lamports:   stake_lamports,
        registered_at:     stake.registered_at,
    });

    Ok(())
}

#[event]
pub struct AgentRegistered {
    pub agent: Pubkey,
    pub staked_lamports: u64,
    pub registered_at: i64,
}
