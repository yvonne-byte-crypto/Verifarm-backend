use anchor_lang::prelude::*;
use crate::state::farmer::{Farmer, FarmerStatus};
use crate::state::loan::{Loan, LoanStatus};
use crate::state::admin::LoanOfficerEntry;
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

    #[account(
        seeds = [LoanOfficerEntry::SEED, loan_officer.key().as_ref()],
        bump = officer_entry.bump,
        constraint = officer_entry.active @ VeriFarmError::UnauthorizedOracle,
        constraint = officer_entry.officer == loan_officer.key() @ VeriFarmError::UnauthorizedOracle,
    )]
    pub officer_entry: Account<'info, LoanOfficerEntry>,

    pub loan_officer: Signer<'info>,
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
