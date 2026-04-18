use anchor_lang::prelude::*;
use crate::state::admin::AdminConfig;

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + AdminConfig::INIT_SPACE,
        seeds = [AdminConfig::SEED],
        bump,
    )]
    pub admin_config: Account<'info, AdminConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeProgram>) -> Result<()> {
    let config = &mut ctx.accounts.admin_config;
    config.admin = ctx.accounts.admin.key();
    config.bump = ctx.bumps.admin_config;
    config.initialized_at = Clock::get()?.unix_timestamp;

    emit!(ProgramInitialized {
        admin: config.admin,
        initialized_at: config.initialized_at,
    });

    Ok(())
}

#[event]
pub struct ProgramInitialized {
    pub admin: Pubkey,
    pub initialized_at: i64,
}
