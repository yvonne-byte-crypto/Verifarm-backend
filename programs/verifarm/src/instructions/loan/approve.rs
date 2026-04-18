use anchor_lang::prelude::*;
use crate::state::loan::{Loan, LoanStatus};
use crate::error::VeriFarmError;

#[derive(Accounts)]
pub struct ApproveLoan<'info> {
    #[account(
        mut,
        seeds = [Loan::SEED, loan.farmer.as_ref(), loan.loan_index.to_le_bytes().as_ref()],
        bump = loan.bump,
    )]
    pub loan: Account<'info, Loan>,

    /// Loan officer with authority to approve
    pub loan_officer: Signer<'info>,
    // TODO: add loan_officer_registry PDA to restrict who can approve
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
