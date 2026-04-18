use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::state::farmer::{Farmer, FarmerStatus};
use crate::state::loan::{Loan, LoanStatus};
use crate::state::risk_oracle::{RiskScore, RiskTier};
use crate::error::VeriFarmError;

pub const MIN_LOAN_USD_CENTS: u64 = 5_000;  // $50 minimum

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApplyForLoanArgs {
    pub amount_usd_cents: u64,
    pub term_days: u16,
    pub interest_bps: u16,
}

#[derive(Accounts)]
#[instruction(args: ApplyForLoanArgs)]
pub struct ApplyForLoan<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Loan::INIT_SPACE,
        seeds = [Loan::SEED, farmer.key().as_ref(), farmer.loan_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub loan: Account<'info, Loan>,

    #[account(
        mut,
        seeds = [Farmer::SEED, authority.key().as_ref()],
        bump = farmer.bump,
        has_one = authority,
    )]
    pub farmer: Account<'info, Farmer>,

    #[account(
        seeds = [RiskScore::SEED, farmer.key().as_ref()],
        bump = risk_score.bump,
    )]
    pub risk_score: Account<'info, RiskScore>,

    /// USDC or stablecoin mint
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ApplyForLoan>, args: ApplyForLoanArgs) -> Result<()> {
    let farmer = &mut ctx.accounts.farmer;
    let risk_score = &ctx.accounts.risk_score;
    let now = Clock::get()?.unix_timestamp;

    require!(
        matches!(farmer.status, FarmerStatus::Verified),
        VeriFarmError::FarmerNotVerified
    );
    require!(
        !risk_score.is_expired(now),
        VeriFarmError::RiskScoreExpired
    );
    require!(
        args.amount_usd_cents >= MIN_LOAN_USD_CENTS,
        VeriFarmError::LoanAmountTooSmall
    );
    require!(
        args.amount_usd_cents <= risk_score.tier.max_loan_usd_cents(),
        VeriFarmError::LoanAmountTooLarge
    );
    require!(
        !matches!(risk_score.tier, RiskTier::Ineligible),
        VeriFarmError::RiskScoreTooLow
    );

    let loan_index = farmer.loan_count;
    farmer.loan_count = farmer.loan_count.checked_add(1).unwrap();

    let loan = &mut ctx.accounts.loan;
    loan.farmer = farmer.key();
    loan.token_mint = ctx.accounts.token_mint.key();
    loan.principal = args.amount_usd_cents;
    loan.outstanding = args.amount_usd_cents;
    loan.interest_bps = args.interest_bps;
    loan.term_days = args.term_days;
    loan.status = LoanStatus::Pending;
    loan.risk_score_at_approval = risk_score.score;
    loan.loan_index = loan_index;
    loan.bump = ctx.bumps.loan;
    loan.applied_at = now;
    loan.approved_at = None;
    loan.disbursed_at = None;
    loan.due_at = None;
    loan.closed_at = None;

    emit!(LoanApplied {
        loan: ctx.accounts.loan.key(),
        farmer: farmer.key(),
        amount_usd_cents: args.amount_usd_cents,
        applied_at: now,
    });

    Ok(())
}

#[event]
pub struct LoanApplied {
    pub loan: Pubkey,
    pub farmer: Pubkey,
    pub amount_usd_cents: u64,
    pub applied_at: i64,
}
