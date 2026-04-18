use anchor_lang::prelude::*;
use crate::state::farmer::{Farmer, FarmerStatus};
use crate::state::loan::{Loan, LoanStatus};
use crate::error::VeriFarmError;

#[derive(Accounts)]
pub struct LiquidateLoan<'info> {
    #[account(
        mut,
        seeds = [Loan::SEED, farmer.key().as_ref(), loan.loan_index.to_le_bytes().as_ref()],
        bump = loan.bump,
        has_one = farmer,
    )]
    pub loan: Account<'info, Loan>,

    #[account(
        mut,
        seeds = [Farmer::SEED, farmer.authority.as_ref()],
        bump = farmer.bump,
    )]
    pub farmer: Account<'info, Farmer>,

    /// Loan officer initiating liquidation
    pub loan_officer: Signer<'info>,
    // TODO: gate with loan_officer_registry PDA
}

pub fn handler(ctx: Context<LiquidateLoan>) -> Result<()> {
    let loan_key = ctx.accounts.loan.key();
    let farmer_key = ctx.accounts.farmer.key();
    let officer_key = ctx.accounts.loan_officer.key();
    let loan = &mut ctx.accounts.loan;
    let now = Clock::get()?.unix_timestamp;

    require!(
        loan.is_defaulted(now),
        VeriFarmError::LoanNotDefaulted
    );

    loan.status = LoanStatus::Liquidated;
    loan.closed_at = Some(now);

    ctx.accounts.farmer.status = FarmerStatus::Suspended;

    emit!(LoanLiquidated {
        loan: loan_key,
        farmer: farmer_key,
        loan_officer: officer_key,
        outstanding: loan.outstanding,
        liquidated_at: now,
    });

    Ok(())
}

#[event]
pub struct LoanLiquidated {
    pub loan: Pubkey,
    pub farmer: Pubkey,
    pub loan_officer: Pubkey,
    pub outstanding: u64,
    pub liquidated_at: i64,
}
