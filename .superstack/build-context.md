# VeriFarm — Build Context

Generated: 2026-04-18

## stack

```json
{
  "framework": "Anchor 0.31.0",
  "language": "Rust (program) + TypeScript (tests/client)",
  "skills_installed": ["programs-anchor", "testing", "security"],
  "mcps_configured": ["helius-mcp", "anchor-mcp"],
  "repos_cloned": [],
  "pattern": "Pattern 4 — On-chain Program (Anchor only)",
  "nft_standard": "MPL Bubblegum 1.4 (cNFTs for livestock tagging)",
  "stablecoin": "USDC (SPL Token)",
  "test_framework": "LiteSVM (unit) + Surfpool (integration)",
  "rpc": "Helius"
}
```

## architecture

```json
{
  "pattern": "On-chain Program — Pattern 4",
  "key_decisions": {
    "livestock_tagging": "cNFTs via Bubblegum — cost-efficient at scale in East Africa vs standard NFTs",
    "risk_scoring": "Off-chain oracle pattern — AI model runs off-chain, trusted signer commits score on-chain",
    "national_id_storage": "SHA-256 hash only — never plaintext on-chain",
    "loan_indexing": "Farmer.loan_count used as loan_index in PDA seed — deterministic, no collision",
    "score_expiry": "30-day TTL on RiskScore — oracle must refresh before each loan approval"
  }
}
```

## build_status

```json
{
  "mvp_complete": false,
  "tests_passing": false,
  "devnet_deployed": false,
  "todos": [
    "Install Rust + Anchor CLI + Solana CLI (see toolchain setup below)",
    "Implement OracleRegistry PDA",
    "Implement LoanOfficerRegistry PDA",
    "Implement update_farmer_status (admin-gated)",
    "Implement Bubblegum CPI in tag_livestock.rs",
    "Wire up USDC token vault + disburse_loan instruction",
    "Add interest accrual logic",
    "Write LiteSVM error-path tests"
  ]
}
```

## toolchain_setup

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Install Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# 3. Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.31.0
avm use 0.31.0

# 4. Generate a dev wallet
solana-keygen new
solana config set --url devnet
solana airdrop 5

# 5. Install Node deps
yarn install

# 6. Build
anchor build

# 7. Test
anchor test
```

## phase_handoff

Next skill: **build-with-claude** — guided MVP implementation
