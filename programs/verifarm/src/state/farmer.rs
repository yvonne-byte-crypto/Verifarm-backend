use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Farmer {
    /// Wallet that owns this farmer account
    pub authority: Pubkey,
    /// National ID hash (SHA-256, stored as bytes — never plaintext on-chain)
    pub national_id_hash: [u8; 32],
    /// Full name (max 64 chars)
    #[max_len(64)]
    pub full_name: String,
    /// Phone number for field agent contact
    #[max_len(20)]
    pub phone: String,
    /// GPS coordinates encoded as fixed-point (lat * 1e6, lng * 1e6)
    pub location_lat: i64,
    pub location_lng: i64,
    /// Verification status
    pub status: FarmerStatus,
    /// Total loans taken (for credit history)
    pub loan_count: u16,
    /// Total loans repaid on time
    pub on_time_repayments: u16,
    /// Latest risk score (cached from RiskScore PDA)
    pub latest_risk_score: u8,
    /// Bump for PDA
    pub bump: u8,
    /// Unix timestamp of registration
    pub registered_at: i64,
}

impl Farmer {
    pub const SEED: &'static [u8] = b"farmer";
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum FarmerStatus {
    Pending,   // Registered, awaiting field verification
    Verified,  // KYC complete, eligible for loans
    Suspended, // Flagged — cannot take new loans
    Blacklisted,
}
