use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::state::admin::AdminConfig;

pub const VAULT_SEED: &[u8] = b"protocol_vault";

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    /// PDA-owned USDC vault — the vault PDA is its own SPL authority
    #[account(
        init,
        payer = admin,
        seeds = [VAULT_SEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = vault,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        seeds = [AdminConfig::SEED],
        bump = admin_config.bump,
        has_one = admin,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeVault>) -> Result<()> {
    emit!(VaultInitialized {
        vault: ctx.accounts.vault.key(),
        mint: ctx.accounts.token_mint.key(),
        admin: ctx.accounts.admin.key(),
    });
    Ok(())
}

#[event]
pub struct VaultInitialized {
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub admin: Pubkey,
}
