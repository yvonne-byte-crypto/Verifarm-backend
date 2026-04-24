use anchor_lang::prelude::*;

#[error_code]
pub enum VeriFarmError {
    #[msg("Farmer is already registered")]
    FarmerAlreadyRegistered,
    #[msg("Farmer is not verified — complete KYC first")]
    FarmerNotVerified,
    #[msg("Risk score has expired — oracle must resubmit")]
    RiskScoreExpired,
    #[msg("Risk score is too low for this loan amount")]
    RiskScoreTooLow,
    #[msg("Risk score was submitted by an unauthorized oracle")]
    UnauthorizedOracle,
    #[msg("Loan is not in the expected state for this operation")]
    InvalidLoanState,
    #[msg("Repayment amount exceeds outstanding balance")]
    OverRepayment,
    #[msg("Loan is not yet past the due date — cannot liquidate")]
    LoanNotDefaulted,
    #[msg("Asset is already verified")]
    AssetAlreadyVerified,
    #[msg("Asset does not belong to this farmer")]
    AssetOwnerMismatch,
    #[msg("Merkle tree is full — create a new tree for livestock tags")]
    MerkleTreeFull,
    #[msg("Name exceeds maximum length")]
    NameTooLong,
    #[msg("Invalid national ID format")]
    InvalidNationalId,
    #[msg("Loan amount below minimum threshold")]
    LoanAmountTooSmall,
    #[msg("Loan amount exceeds maximum allowed for this risk tier")]
    LoanAmountTooLarge,

    // ── Agent staking ──────────────────────────────────────────────────────
    #[msg("Agent is not registered — call register_agent first")]
    AgentNotRegistered,
    #[msg("Agent is already registered")]
    AgentAlreadyRegistered,
    #[msg("Agent is suspended and cannot submit verifications")]
    AgentSuspended,
    #[msg("Stake amount is below the minimum required (0.1 SOL)")]
    InsufficientStake,
    #[msg("The 72-hour dispute window has expired for this verification")]
    DisputeWindowExpired,
    #[msg("This verification still has an active dispute window — stake is locked")]
    DisputeWindowActive,
    #[msg("Verification is not in Pending status")]
    VerificationNotPending,
    #[msg("Verification is not in Disputed status")]
    VerificationNotDisputed,
    #[msg("Agent has active verifications pending — resolve disputes before withdrawing")]
    ActiveVerificationsPending,
    #[msg("Caller is not authorised to perform this action")]
    Unauthorized,
}
