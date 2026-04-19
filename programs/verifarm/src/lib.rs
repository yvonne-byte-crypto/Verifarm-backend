use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod instructions;

use instructions::farmer::*;
use instructions::asset::*;
use instructions::risk::*;
use instructions::loan::*;
use instructions::admin::*;

declare_id!("9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N");

#[program]
pub mod verifarm {
    use super::*;

    // ── Admin ────────────────────────────────────────────────────────────────
    pub fn initialize_program(ctx: Context<InitializeProgram>) -> Result<()> {
        instructions::admin::initialize_program::handler(ctx)
    }

    pub fn register_oracle(ctx: Context<RegisterOracle>, oracle: Pubkey) -> Result<()> {
        instructions::admin::manage_oracle::register_oracle_handler(ctx, oracle)
    }

    pub fn revoke_oracle(ctx: Context<RevokeOracle>, oracle: Pubkey) -> Result<()> {
        instructions::admin::manage_oracle::revoke_oracle_handler(ctx, oracle)
    }

    pub fn register_officer(ctx: Context<RegisterOfficer>, officer: Pubkey) -> Result<()> {
        instructions::admin::manage_officer::register_officer_handler(ctx, officer)
    }

    pub fn revoke_officer(ctx: Context<RevokeOfficer>, officer: Pubkey) -> Result<()> {
        instructions::admin::manage_officer::revoke_officer_handler(ctx, officer)
    }

    pub fn update_farmer_status(ctx: Context<UpdateFarmerStatus>, args: UpdateFarmerStatusArgs) -> Result<()> {
        instructions::admin::update_farmer_status::handler(ctx, args)
    }

    // ── Farmer Registry ──────────────────────────────────────────────────────
    pub fn register_farmer(ctx: Context<RegisterFarmer>, args: RegisterFarmerArgs) -> Result<()> {
        instructions::farmer::register::handler(ctx, args)
    }

    pub fn update_farmer_profile(ctx: Context<UpdateFarmerProfile>, args: UpdateFarmerProfileArgs) -> Result<()> {
        instructions::farmer::update_profile::handler(ctx, args)
    }

    // ── Asset Verification ───────────────────────────────────────────────────
    pub fn verify_asset(ctx: Context<VerifyAsset>, args: VerifyAssetArgs) -> Result<()> {
        instructions::asset::verify::handler(ctx, args)
    }

    pub fn tag_livestock(ctx: Context<TagLivestock>, args: TagLivestockArgs) -> Result<()> {
        instructions::asset::tag_livestock::handler(ctx, args)
    }

    // ── Risk Oracle ──────────────────────────────────────────────────────────
    pub fn submit_risk_score(ctx: Context<SubmitRiskScore>, args: SubmitRiskScoreArgs) -> Result<()> {
        instructions::risk::submit_score::handler(ctx, args)
    }

    // ── Loan Management ──────────────────────────────────────────────────────
    pub fn apply_for_loan(ctx: Context<ApplyForLoan>, args: ApplyForLoanArgs) -> Result<()> {
        instructions::loan::apply::handler(ctx, args)
    }

    pub fn approve_loan(ctx: Context<ApproveLoan>) -> Result<()> {
        instructions::loan::approve::handler(ctx)
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::loan::initialize_vault::handler(ctx)
    }

    pub fn disburse_loan(ctx: Context<DisburseLoan>) -> Result<()> {
        instructions::loan::disburse::handler(ctx)
    }

    pub fn repay_loan(ctx: Context<RepayLoan>, args: RepayLoanArgs) -> Result<()> {
        instructions::loan::repay::handler(ctx, args)
    }

    pub fn liquidate_loan(ctx: Context<LiquidateLoan>) -> Result<()> {
        instructions::loan::liquidate::handler(ctx)
    }
}
