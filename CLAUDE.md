# VeriFarm — Anchor Program

Agricultural micro-lending platform for East Africa smallholder farmers built on Solana.

## What this program does

- **Farmer Registry**: On-chain identity with hashed national ID, GPS location, and KYC status
- **Asset Verification**: Field officers verify land, equipment, and livestock assets as loan collateral
- **Livestock Tagging**: Compressed NFTs (cNFTs) via MPL Bubblegum — one NFT per animal, stored on a shared merkle tree
- **AI Risk Scoring**: Off-chain model submits scores to a trusted oracle PDA; scores expire after 30 days
- **Loan Management**: Full lifecycle — apply → approve → disburse → repay → liquidate
- **Agent Staking**: Anti-fraud layer — field agents stake SOL before submitting verifications; bad actors get slashed

## Program accounts (PDAs)

| Account | Seeds | Purpose |
|---|---|---|
| `Farmer` | `["farmer", authority]` | Farmer identity and status |
| `Asset` | `["asset", farmer_pda, asset_index]` | Verified collateral asset |
| `RiskScore` | `["risk_score", farmer_pda]` | Latest AI score (refreshed by oracle) |
| `Loan` | `["loan", farmer_pda, loan_index]` | Individual loan instance |
| `AgentStake` | `["agent_stake", agent]` | Agent's staked SOL + status (Active/Suspended) |
| `VerificationRecord` | `["verification_record", agent, farmer]` | On-chain record of a field verification |
| `TreasuryVault` | `["treasury"]` | Collects slashed lamports from bad agents |

## Agent Staking (anti-fraud)

Field agents must lock **0.1 SOL minimum** to register. This stake is slashable if a verification is disputed and confirmed fraudulent.

**Lifecycle:**
```
register_agent (stake SOL)
  → submit_verification (creates VerificationRecord, status=Pending)
    → dispute_verification (within 72-hour window, status=Disputed)
      → confirm_dispute (admin-only, slashes stake to treasury, status=Slashed, agent=Suspended)
    OR → dispute window expires → status stays Pending (cleared implicitly)
  → withdraw_stake (only if Active + active_verifications == 0)
```

**Key constants:**
- `MIN_STAKE_LAMPORTS = 100_000_000` (0.1 SOL)
- `DISPUTE_WINDOW_SECS = 259_200` (72 hours)

**Instructions:**

| Instruction | Who calls | What it does |
|---|---|---|
| `register_agent` | Any signer | Stakes SOL, creates `AgentStake` PDA |
| `submit_verification` | Active agent | Creates `VerificationRecord`, increments `active_verifications` |
| `dispute_verification` | Admin or active agent | Flags a pending verification within dispute window |
| `confirm_dispute` | Admin only | Slashes stake → treasury, suspends agent |
| `withdraw_stake` | Active agent | Closes `AgentStake`, returns lamports (requires 0 pending verifications) |

## Key invariants

- Farmers must be `FarmerStatus::Verified` before applying for loans
- `RiskScore` expires after 30 days — oracle must resubmit before loan approval
- Loan amounts are capped by `RiskTier::max_loan_usd_cents()` (Prime: $5k, Standard: $2k, SubPrime: $500)
- `Farmer.loan_count` is the loan index — increment atomically in `apply_for_loan`
- Liquidation only allowed when `loan.is_defaulted(now)` returns true

## TODOs before devnet

- [ ] Implement Bubblegum CPI in `tag_livestock.rs`
- [ ] Wire up USDC token vault for disburse instruction
- [ ] Add interest accrual (simple daily interest or balloon payment)
- [ ] Link `submit_verification` to specific `Asset` PDAs (currently just farmer-level)
- [ ] Add `clearVerification` instruction for post-window cleanup

## Build & test

```bash
# Install toolchain (first time)
# See SETUP.md for Rust + Anchor + Solana CLI install

anchor build
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Stack

- **Framework**: Anchor 0.31.0
- **Livestock tags**: MPL Bubblegum 1.4 (cNFTs)
- **Stablecoin**: USDC (SPL Token)
- **Test framework**: anchor-bankrun (unit, 33 tests) + Surfpool (integration)
- **RPC**: Helius (DAS API for cNFT queries)
- **Risk scoring**: Off-chain AI model → on-chain oracle PDA

## Security notes

- National IDs are **never stored plaintext** — only SHA-256 hash on-chain
- Oracle authorization is currently a stub — implement `OracleRegistry` before mainnet
- Loan officer authorization is a stub — implement `LoanOfficerRegistry` before mainnet
- All arithmetic uses `checked_add`/`checked_sub` to prevent overflow
