use anchor_lang::prelude::*;
use crate::state::farmer::{Farmer, FarmerStatus};
use crate::state::asset::{Asset, AssetType};
use crate::error::VeriFarmError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VerifyAssetArgs {
    pub asset_type: AssetType,
    pub description: String,
    pub value_usd_cents: u64,
    /// Unique index for this asset under this farmer (used in PDA seed)
    pub asset_index: u16,
}

#[derive(Accounts)]
#[instruction(args: VerifyAssetArgs)]
pub struct VerifyAsset<'info> {
    #[account(
        init,
        payer = field_officer,
        space = 8 + Asset::INIT_SPACE,
        seeds = [Asset::SEED, farmer.key().as_ref(), args.asset_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub asset: Account<'info, Asset>,

    #[account(
        seeds = [Farmer::SEED, farmer.authority.as_ref()],
        bump = farmer.bump,
    )]
    pub farmer: Account<'info, Farmer>,

    /// Field officer / loan officer who performed physical verification
    #[account(mut)]
    pub field_officer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VerifyAsset>, args: VerifyAssetArgs) -> Result<()> {
    let asset = &mut ctx.accounts.asset;
    let now = Clock::get()?.unix_timestamp;

    asset.farmer = ctx.accounts.farmer.key();
    asset.asset_type = args.asset_type;
    asset.description = args.description;
    asset.value_usd_cents = args.value_usd_cents;
    asset.verified = true;
    asset.verified_by = Some(ctx.accounts.field_officer.key());
    asset.verified_at = Some(now);
    asset.livestock_tag = None;
    asset.bump = ctx.bumps.asset;
    asset.registered_at = now;

    emit!(AssetVerified {
        asset: ctx.accounts.asset.key(),
        farmer: ctx.accounts.farmer.key(),
        field_officer: ctx.accounts.field_officer.key(),
        value_usd_cents: args.value_usd_cents,
        verified_at: now,
    });

    Ok(())
}

#[event]
pub struct AssetVerified {
    pub asset: Pubkey,
    pub farmer: Pubkey,
    pub field_officer: Pubkey,
    pub value_usd_cents: u64,
    pub verified_at: i64,
}
