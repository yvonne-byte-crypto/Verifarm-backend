use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Asset {
    /// Farmer this asset belongs to
    pub farmer: Pubkey,
    /// Asset type
    pub asset_type: AssetType,
    /// Human-readable description
    #[max_len(128)]
    pub description: String,
    /// Estimated value in USD cents (e.g., 150000 = $1,500.00)
    pub value_usd_cents: u64,
    /// Verification status
    pub verified: bool,
    /// Field agent who verified (pubkey of the verifying officer)
    pub verified_by: Option<Pubkey>,
    /// Timestamp of verification
    pub verified_at: Option<i64>,
    /// For livestock: the cNFT asset_id from Bubblegum (leaf hash)
    pub livestock_tag: Option<[u8; 32]>,
    pub bump: u8,
    pub registered_at: i64,
}

impl Asset {
    pub const SEED: &'static [u8] = b"asset";
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum AssetType {
    Land,
    Equipment,
    Livestock,
    Crop,
    Other,
}
