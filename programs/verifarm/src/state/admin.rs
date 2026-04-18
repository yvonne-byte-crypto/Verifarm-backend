use anchor_lang::prelude::*;

/// Singleton PDA — one per program deployment.
/// Holds the admin pubkey and oracle whitelist.
#[account]
#[derive(InitSpace)]
pub struct AdminConfig {
    pub admin: Pubkey,
    pub bump: u8,
    pub initialized_at: i64,
}

impl AdminConfig {
    pub const SEED: &'static [u8] = b"admin_config";
}

/// One PDA per oracle — allows fine-grained revocation.
#[account]
#[derive(InitSpace)]
pub struct OracleEntry {
    pub oracle: Pubkey,
    pub added_by: Pubkey,
    pub active: bool,
    pub bump: u8,
    pub added_at: i64,
}

impl OracleEntry {
    pub const SEED: &'static [u8] = b"oracle_entry";
}

/// One PDA per loan officer — allows fine-grained revocation.
#[account]
#[derive(InitSpace)]
pub struct LoanOfficerEntry {
    pub officer: Pubkey,
    pub added_by: Pubkey,
    pub active: bool,
    pub bump: u8,
    pub added_at: i64,
}

impl LoanOfficerEntry {
    pub const SEED: &'static [u8] = b"loan_officer_entry";
}
