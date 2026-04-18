use anchor_lang::prelude::*;
use crate::state::farmer::Farmer;
use crate::state::risk_oracle::{RiskScore, RiskTier};
use crate::error::VeriFarmError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SubmitRiskScoreArgs {
    pub score: u8,
    pub confidence: u8,
    pub model_version: String,
}

#[derive(Accounts)]
pub struct SubmitRiskScore<'info> {
    #[account(
        init_if_needed,
        payer = oracle,
        space = 8 + RiskScore::INIT_SPACE,
        seeds = [RiskScore::SEED, farmer.key().as_ref()],
        bump,
    )]
    pub risk_score: Account<'info, RiskScore>,

    #[account(
        mut,
        seeds = [Farmer::SEED, farmer.authority.as_ref()],
        bump = farmer.bump,
    )]
    pub farmer: Account<'info, Farmer>,

    /// Trusted oracle keypair — validated against oracle_registry PDA (TODO: add registry)
    #[account(mut)]
    pub oracle: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SubmitRiskScore>, args: SubmitRiskScoreArgs) -> Result<()> {
    require!(args.score <= 100, VeriFarmError::RiskScoreTooLow); // reusing as range check
    require!(args.confidence <= 100, VeriFarmError::RiskScoreTooLow);

    let now = Clock::get()?.unix_timestamp;
    let tier = RiskTier::from_score(args.score);

    let risk_score = &mut ctx.accounts.risk_score;
    risk_score.farmer = ctx.accounts.farmer.key();
    risk_score.score = args.score;
    risk_score.confidence = args.confidence;
    risk_score.tier = tier;
    risk_score.oracle = ctx.accounts.oracle.key();
    risk_score.scored_at = now;
    risk_score.model_version = args.model_version;
    risk_score.bump = ctx.bumps.risk_score;

    // Cache score on farmer for quick lookup
    ctx.accounts.farmer.latest_risk_score = args.score;

    emit!(RiskScoreSubmitted {
        farmer: ctx.accounts.farmer.key(),
        score: args.score,
        confidence: args.confidence,
        oracle: ctx.accounts.oracle.key(),
        scored_at: now,
    });

    Ok(())
}

#[event]
pub struct RiskScoreSubmitted {
    pub farmer: Pubkey,
    pub score: u8,
    pub confidence: u8,
    pub oracle: Pubkey,
    pub scored_at: i64,
}
