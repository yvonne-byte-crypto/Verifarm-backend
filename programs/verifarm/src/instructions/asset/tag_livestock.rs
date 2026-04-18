use anchor_lang::prelude::*;
use crate::state::asset::Asset;

/// Mints a compressed NFT (cNFT) via MPL Bubblegum to represent a livestock animal.
/// The leaf hash is stored in the Asset PDA for provenance tracking.
///
/// Bubblegum CPI requires the merkle tree, tree authority, log wrapper, and
/// account compression program — all passed as remaining accounts for flexibility
/// across different tree configurations.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TagLivestockArgs {
    pub asset_index: u16,
    /// Livestock metadata: name, species, ear tag number
    pub name: String,
    pub symbol: String,
    pub uri: String, // IPFS/Arweave metadata URI (photo + health records)
}

#[derive(Accounts)]
#[instruction(args: TagLivestockArgs)]
pub struct TagLivestock<'info> {
    #[account(
        mut,
        seeds = [Asset::SEED, asset.farmer.as_ref(), args.asset_index.to_le_bytes().as_ref()],
        bump = asset.bump,
    )]
    pub asset: Account<'info, Asset>,

    /// Field officer minting the tag
    #[account(mut)]
    pub field_officer: Signer<'info>,

    /// CHECK: Bubblegum merkle tree — validated by Bubblegum program
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,

    /// CHECK: Bubblegum tree authority PDA
    pub tree_authority: UncheckedAccount<'info>,

    /// CHECK: SPL Noop program for logging
    pub log_wrapper: UncheckedAccount<'info>,

    /// CHECK: SPL Account Compression program
    pub compression_program: UncheckedAccount<'info>,

    /// CHECK: MPL Bubblegum program
    pub bubblegum_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TagLivestock>, args: TagLivestockArgs) -> Result<()> {
    // TODO: CPI into mpl_bubblegum::instructions::MintV1Cpi
    // The leaf hash returned by Bubblegum should be stored in asset.livestock_tag
    //
    // Pattern:
    //   let cpi_accounts = MintV1CpiAccounts { ... };
    //   let metadata = MetadataArgs { name, symbol, uri, ... };
    //   MintV1Cpi::new(bubblegum_program, cpi_accounts, metadata).invoke()?;
    //   asset.livestock_tag = Some(leaf_hash);

    msg!("TagLivestock: Bubblegum CPI stub — implement with mpl-bubblegum 1.4");
    msg!("Asset: {}, Tree: {}", ctx.accounts.asset.key(), ctx.accounts.merkle_tree.key());

    Ok(())
}
