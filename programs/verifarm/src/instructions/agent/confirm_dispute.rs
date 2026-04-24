use anchor_lang::prelude::*;
use crate::state::admin::AdminConfig;
use crate::state::agent::{AgentStake, AgentStatus, VerificationRecord, VerificationStatus, TreasuryVault};
use crate::error::VeriFarmError;

#[derive(Accounts)]
pub struct ConfirmDispute<'info> {
    #[account(
        mut,
        seeds = [VerificationRecord::SEED, verification_record.agent.as_ref(), verification_record.farmer.as_ref()],
        bump = verification_record.bump,
    )]
    pub verification_record: Account<'info, VerificationRecord>,

    #[account(
        mut,
        seeds = [AgentStake::SEED, verification_record.agent.as_ref()],
        bump = agent_stake.bump,
    )]
    pub agent_stake: Account<'info, AgentStake>,

    /// Treasury accumulates slashed lamports.
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + TreasuryVault::INIT_SPACE,
        seeds = [TreasuryVault::SEED],
        bump,
    )]
    pub treasury: Account<'info, TreasuryVault>,

    #[account(
        seeds = [AdminConfig::SEED],
        bump = admin_config.bump,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    #[account(mut, constraint = admin.key() == admin_config.admin @ VeriFarmError::Unauthorized)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ConfirmDispute>) -> Result<()> {
    require!(
        ctx.accounts.verification_record.status == VerificationStatus::Disputed,
        VeriFarmError::VerificationNotDisputed
    );

    let slash_amount = ctx.accounts.agent_stake.staked_lamports;

    // Move staked lamports from agent_stake PDA to treasury PDA.
    // Both are program-owned accounts so direct lamport manipulation is safe.
    if slash_amount > 0 {
        **ctx.accounts.agent_stake.to_account_info().try_borrow_mut_lamports()? -= slash_amount;
        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += slash_amount;
        ctx.accounts.treasury.total_slashed_lamports =
            ctx.accounts.treasury.total_slashed_lamports.saturating_add(slash_amount);
    }

    // Suspend agent
    ctx.accounts.agent_stake.status           = AgentStatus::Suspended;
    ctx.accounts.agent_stake.staked_lamports  = 0;
    ctx.accounts.agent_stake.active_verifications =
        ctx.accounts.agent_stake.active_verifications.saturating_sub(1);

    ctx.accounts.verification_record.status = VerificationStatus::Slashed;

    emit!(DisputeConfirmed {
        verification_record: ctx.accounts.verification_record.key(),
        agent:               ctx.accounts.agent_stake.agent,
        slash_amount,
        confirmed_by:        ctx.accounts.admin.key(),
        confirmed_at:        Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct DisputeConfirmed {
    pub verification_record: Pubkey,
    pub agent: Pubkey,
    pub slash_amount: u64,
    pub confirmed_by: Pubkey,
    pub confirmed_at: i64,
}
