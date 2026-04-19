use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::state::loan::{Loan, LoanStatus};
use crate::state::farmer::Farmer;
use crate::state::admin::LoanOfficerEntry;
use crate::error::VeriFarmError;
use super::initialize_vault::VAULT_SEED;

#[derive(Accounts)]
pub struct DisburseLoan<'info> {
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

    /// Protocol USDC vault — signs via PDA seeds
    #[account(
        mut,
        seeds = [VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = vault,
    )]
    pub vault: Account<'info, TokenAccount>,

    /// Farmer's USDC token account to receive funds
    #[account(
        mut,
        token::mint = token_mint,
    )]
    pub farmer_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    /// Active loan officer must be registered
    #[account(
        seeds = [LoanOfficerEntry::SEED, loan_officer.key().as_ref()],
        bump = officer_entry.bump,
        constraint = officer_entry.active @ VeriFarmError::UnauthorizedOracle,
        constraint = officer_entry.officer == loan_officer.key() @ VeriFarmError::UnauthorizedOracle,
    )]
    pub officer_entry: Account<'info, LoanOfficerEntry>,

    pub loan_officer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DisburseLoan>) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    let now = Clock::get()?.unix_timestamp;

    require!(
        matches!(loan.status, LoanStatus::Approved),
        VeriFarmError::InvalidLoanState
    );

    let mint_key = ctx.accounts.token_mint.key();
    let vault_bump = ctx.bumps.vault;
    let vault_seeds: &[&[&[u8]]] = &[&[VAULT_SEED, mint_key.as_ref(), &[vault_bump]]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.farmer_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            vault_seeds,
        ),
        loan.principal,
    )?;

    loan.status = LoanStatus::Active;
    loan.disbursed_at = Some(now);
    loan.due_at = Some(now + (loan.term_days as i64) * 86_400);

    let loan_key = loan.key();
    let farmer_key = ctx.accounts.farmer.key();

    emit!(LoanDisbursed {
        loan: loan_key,
        farmer: farmer_key,
        amount: loan.principal,
        due_at: loan.due_at.unwrap(),
        disbursed_at: now,
    });

    Ok(())
}

#[event]
pub struct LoanDisbursed {
    pub loan: Pubkey,
    pub farmer: Pubkey,
    pub amount: u64,
    pub due_at: i64,
    pub disbursed_at: i64,
}
