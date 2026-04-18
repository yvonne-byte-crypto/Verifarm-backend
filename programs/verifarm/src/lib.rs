use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod instructions;

use instructions::farmer::*;
use instructions::asset::*;
use instructions::risk::*;
use instructions::loan::*;

declare_id!("9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N");

#[program]
pub mod verifarm {
    use super::*;

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

    pub fn repay_loan(ctx: Context<RepayLoan>, args: RepayLoanArgs) -> Result<()> {
        instructions::loan::repay::handler(ctx, args)
    }

    pub fn liquidate_loan(ctx: Context<LiquidateLoan>) -> Result<()> {
        instructions::loan::liquidate::handler(ctx)
    }
}
