use anchor_lang::prelude::*;
use crate::state::loan::{Loan, LoanStatus};
use crate::state::admin::LoanOfficerEntry;
use crate::error::VeriFarmError;

#[derive(Accounts)]
pub struct ApproveLoan<'info> {
    #[account(
        mut,
        seeds = [Loan::SEED, loan.farmer.as_ref(), loan.loan_index.to_le_bytes().as_ref()],
        bump = loan.bump,
    )]
    pub loan: Account<'info, Loan>,

    #[account(
        seeds = [LoanOfficerEntry::SEED, loan_officer.key().as_ref()],
        bump = officer_entry.bump,
        constraint = officer_entry.active @ VeriFarmError::UnauthorizedOracle,
        constraint = officer_entry.officer == loan_officer.key() @ VeriFarmError::UnauthorizedOracle,
    )]
    pub officer_entry: Account<'info, LoanOfficerEntry>,

    pub loan_officer: Signer<'info>,
}

pub fn handler(ctx: Context<ApproveLoan>) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    let now = Clock::get()?.unix_timestamp;

    require!(
        matches!(loan.status, LoanStatus::Pending),
        VeriFarmError::InvalidLoanState
    );

    loan.status = LoanStatus::Approved;
    loan.approved_at = Some(now);

    emit!(LoanApproved {
        loan: ctx.accounts.loan.key(),
        loan_officer: ctx.accounts.loan_officer.key(),
        approved_at: now,
    });

    Ok(())
}

#[event]
pub struct LoanApproved {
    pub loan: Pubkey,
    pub loan_officer: Pubkey,
    pub approved_at: i64,
}
