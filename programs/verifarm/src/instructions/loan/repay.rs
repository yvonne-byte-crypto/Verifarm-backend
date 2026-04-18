use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::farmer::Farmer;
use crate::state::loan::{Loan, LoanStatus};
use crate::error::VeriFarmError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RepayLoanArgs {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct RepayLoan<'info> {
    #[account(
        mut,
        seeds = [Loan::SEED, farmer.key().as_ref(), loan.loan_index.to_le_bytes().as_ref()],
        bump = loan.bump,
        has_one = farmer,
    )]
    pub loan: Account<'info, Loan>,

    #[account(
        mut,
        seeds = [Farmer::SEED, authority.key().as_ref()],
        bump = farmer.bump,
        has_one = authority,
    )]
    pub farmer: Account<'info, Farmer>,

    #[account(mut)]
    pub farmer_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub protocol_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RepayLoan>, args: RepayLoanArgs) -> Result<()> {
    let loan_key = ctx.accounts.loan.key();
    let loan = &mut ctx.accounts.loan;

    require!(
        matches!(loan.status, LoanStatus::Active),
        VeriFarmError::InvalidLoanState
    );
    require!(
        args.amount <= loan.outstanding,
        VeriFarmError::OverRepayment
    );

    // Transfer USDC from farmer to protocol vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.farmer_token_account.to_account_info(),
                to: ctx.accounts.protocol_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        args.amount,
    )?;

    loan.outstanding = loan.outstanding.checked_sub(args.amount).unwrap();

    if loan.outstanding == 0 {
        loan.status = LoanStatus::Repaid;
        loan.closed_at = Some(Clock::get()?.unix_timestamp);
        ctx.accounts.farmer.on_time_repayments =
            ctx.accounts.farmer.on_time_repayments.saturating_add(1);
    }

    emit!(LoanRepaid {
        loan: loan_key,
        amount: args.amount,
        remaining: loan.outstanding,
    });

    Ok(())
}

#[event]
pub struct LoanRepaid {
    pub loan: Pubkey,
    pub amount: u64,
    pub remaining: u64,
}
