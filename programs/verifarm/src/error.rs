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
}
