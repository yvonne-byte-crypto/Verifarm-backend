use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Loan {
    pub farmer: Pubkey,
    /// USDC mint (or other stablecoin)
    pub token_mint: Pubkey,
    /// Principal in token base units
    pub principal: u64,
    /// Outstanding balance (principal + accrued interest)
    pub outstanding: u64,
    /// Annual interest rate in basis points (e.g., 1500 = 15%)
    pub interest_bps: u16,
    /// Loan term in days
    pub term_days: u16,
    /// State machine
    pub status: LoanStatus,
    /// Risk score snapshot at approval time
    pub risk_score_at_approval: u8,
    /// Loan index for this farmer (used in PDA seed)
    pub loan_index: u16,
    pub bump: u8,
    pub applied_at: i64,
    pub approved_at: Option<i64>,
    pub disbursed_at: Option<i64>,
    pub due_at: Option<i64>,
    pub closed_at: Option<i64>,
}

impl Loan {
    pub const SEED: &'static [u8] = b"loan";

    pub fn is_defaulted(&self, now: i64) -> bool {
        matches!(self.status, LoanStatus::Active)
            && self.due_at.map(|d| now > d).unwrap_or(false)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum LoanStatus {
    Pending,    // Applied, awaiting officer review
    Approved,   // Approved, not yet disbursed
    Active,     // Funds disbursed, repayment in progress
    Repaid,     // Fully repaid
    Defaulted,  // Missed due date
    Liquidated, // Assets seized post-default
    Rejected,   // Officer rejected the application
}
