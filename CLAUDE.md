# VeriFarm — Anchor Program

Agricultural micro-lending platform for East Africa smallholder farmers built on Solana.

## What this program does

- **Farmer Registry**: On-chain identity with hashed national ID, GPS location, and KYC status
- **Asset Verification**: Field officers verify land, equipment, and livestock assets as loan collateral
- **Livestock Tagging**: Compressed NFTs (cNFTs) via MPL Bubblegum — one NFT per animal, stored on a shared merkle tree
- **AI Risk Scoring**: Off-chain model submits scores to a trusted oracle PDA; scores expire after 30 days
- **Loan Management**: Full lifecycle — apply → approve → disburse → repay → liquidate

## Program accounts (PDAs)

| Account | Seeds | Purpose |
|---|---|---|
| `Farmer` | `["farmer", authority]` | Farmer identity and status |
| `Asset` | `["asset", farmer_pda, asset_index]` | Verified collateral asset |
| `RiskScore` | `["risk_score", farmer_pda]` | Latest AI score (refreshed by oracle) |
| `Loan` | `["loan", farmer_pda, loan_index]` | Individual loan instance |

## Key invariants

- Farmers must be `FarmerStatus::Verified` before applying for loans
- `RiskScore` expires after 30 days — oracle must resubmit before loan approval
- Loan amounts are capped by `RiskTier::max_loan_usd_cents()` (Prime: $5k, Standard: $2k, SubPrime: $500)
- `Farmer.loan_count` is the loan index — increment atomically in `apply_for_loan`
- Liquidation only allowed when `loan.is_defaulted(now)` returns true

## TODOs before devnet

- [ ] Add `OracleRegistry` PDA — whitelist of authorized oracle pubkeys
- [ ] Add `LoanOfficerRegistry` PDA — whitelist of authorized field officers
- [ ] Add `update_farmer_status` instruction (admin-only, gated by admin PDA)
- [ ] Implement Bubblegum CPI in `tag_livestock.rs`
- [ ] Wire up USDC token vault for disburse instruction
- [ ] Add `disburse_loan` instruction
- [ ] Add interest accrual (simple daily interest or balloon payment)
- [ ] Write LiteSVM unit tests for all error paths

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
- **Test framework**: LiteSVM (unit) + Surfpool (integration)
- **RPC**: Helius (DAS API for cNFT queries)
- **Risk scoring**: Off-chain AI model → on-chain oracle PDA

## Security notes

- National IDs are **never stored plaintext** — only SHA-256 hash on-chain
- Oracle authorization is currently a stub — implement `OracleRegistry` before mainnet
- Loan officer authorization is a stub — implement `LoanOfficerRegistry` before mainnet
- All arithmetic uses `checked_add`/`checked_sub` to prevent overflow
