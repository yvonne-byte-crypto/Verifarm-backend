use anchor_lang::prelude::*;

/// Stores the latest AI risk score for a farmer, written by a trusted oracle.
/// Score expires after SCORE_VALIDITY_SECONDS — oracle must refresh before loan approval.
#[account]
#[derive(InitSpace)]
pub struct RiskScore {
    pub farmer: Pubkey,
    /// Score 0–100. Higher = less risky.
    pub score: u8,
    /// Confidence 0–100 from the AI model
    pub confidence: u8,
    /// Risk tier derived from score
    pub tier: RiskTier,
    /// Oracle that submitted this score
    pub oracle: Pubkey,
    /// Unix timestamp when this score was computed
    pub scored_at: i64,
    /// Model version string (e.g., "v2.1.0")
    #[max_len(16)]
    pub model_version: String,
    pub bump: u8,
}

impl RiskScore {
    pub const SEED: &'static [u8] = b"risk_score";
    /// Score is valid for 30 days
    pub const SCORE_VALIDITY_SECONDS: i64 = 30 * 24 * 60 * 60;

    pub fn is_expired(&self, now: i64) -> bool {
        now - self.scored_at > Self::SCORE_VALIDITY_SECONDS
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum RiskTier {
    /// Score 80–100: max loan $5,000
    Prime,
    /// Score 60–79: max loan $2,000
    Standard,
    /// Score 40–59: max loan $500
    SubPrime,
    /// Score 0–39: loan denied
    Ineligible,
}

impl RiskTier {
    pub fn from_score(score: u8) -> Self {
        match score {
            80..=100 => RiskTier::Prime,
            60..=79  => RiskTier::Standard,
            40..=59  => RiskTier::SubPrime,
            _        => RiskTier::Ineligible,
        }
    }

    /// Maximum loan in USD cents
    pub fn max_loan_usd_cents(&self) -> u64 {
        match self {
            RiskTier::Prime     => 500_000,  // $5,000
            RiskTier::Standard  => 200_000,  // $2,000
            RiskTier::SubPrime  => 50_000,   // $500
            RiskTier::Ineligible => 0,
        }
    }
}
