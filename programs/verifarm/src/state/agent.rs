use anchor_lang::prelude::*;

pub const MIN_STAKE_LAMPORTS: u64 = 100_000_000; // 0.1 SOL
pub const DISPUTE_WINDOW_SECS: i64 = 72 * 60 * 60; // 72 hours

/// One PDA per registered field agent — holds their stake and status.
#[account]
#[derive(InitSpace)]
pub struct AgentStake {
    pub agent: Pubkey,
    /// Lamports staked (beyond rent). Slashed to zero on confirmed fraud.
    pub staked_lamports: u64,
    /// Count of verifications still in their dispute window.
    pub active_verifications: u32,
    pub status: AgentStatus,
    pub bump: u8,
    pub registered_at: i64,
}

impl AgentStake {
    pub const SEED: &'static [u8] = b"agent_stake";
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum AgentStatus {
    Active,
    Suspended,
}

/// One PDA per (agent, farmer) verification submission.
#[account]
#[derive(InitSpace)]
pub struct VerificationRecord {
    pub agent: Pubkey,
    pub farmer: Pubkey,
    pub submitted_at: i64,
    /// Unix timestamp when the 72-hour dispute window closes.
    pub dispute_window_ends: i64,
    pub status: VerificationStatus,
    pub disputed_by: Option<Pubkey>,
    pub disputed_at: Option<i64>,
    pub bump: u8,
}

impl VerificationRecord {
    pub const SEED: &'static [u8] = b"verification_record";
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum VerificationStatus {
    Pending,
    Disputed,
    Cleared,  // dispute window passed with no challenge
    Slashed,  // fraud confirmed — agent slashed
}

/// Singleton PDA — accumulates slashed agent stake.
#[account]
#[derive(InitSpace)]
pub struct TreasuryVault {
    pub bump: u8,
    pub total_slashed_lamports: u64,
}

impl TreasuryVault {
    pub const SEED: &'static [u8] = b"treasury";
}
