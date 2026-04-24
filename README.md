# 🌾 VeriFarm — Solana Anchor Backend

> VeriFarm is the trust bridge between smallholder farmers and lending institutions 
> verifying assets on one side, delivering confident lending decisions on the other,
> and managing the full loan lifecycle on-chain until every loan is repaid.

![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?style=flat&logo=solana)
![Anchor](https://img.shields.io/badge/Anchor-0.31.0-blue)
![Tests](https://img.shields.io/badge/Tests-33%20Passing-brightgreen)
![Status](https://img.shields.io/badge/Status-Live%20on%20Devnet-brightgreen)
![Built With](https://img.shields.io/badge/Built%20with-Claude%20Code%20%2B%20solana.new-orange)

---

## ⛓️ Deployed Program

**Program ID:** `9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N`  
**Network:** Devnet  
**Framework:** Anchor 0.31.0  
**Tests:** 33 bankrun tests passing  

🔗 [View on Solana Explorer](https://explorer.solana.com/address/9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N?cluster=devnet)  
🖥️ [Frontend Dashboard](https://verifarm-frontend.vercel.app)  
📁 [Frontend Repo](https://github.com/yvonne-byte-crypto/verifarm-frontend)

---

## 💰 Capital Model

VeriFarm does not hold or lend capital directly.

**How it works:**
- Licensed MFIs and SACCOs in Kenya and Tanzania 
  provide the regulated loan capital
- VeriFarm provides the verification and scoring rails
- Initial pilot capital: partnering with 2 target MFIs 
  in Tanzania and Kenya for the Mainnet launch
- VeriFarm earns an oracle attestation fee per verified 
  farmer — not a lending margin

**Why this matters:**
- Regulatory compliance sits with licensed partners
- VeriFarm never touches customer funds
- DeFi liquidity can flow through compliant 
  institutional layer as we scale
- Farmers get credit. MFIs get reach. 
  VeriFarm gets the oracle fee.

--- 

## 🏗️ Milestone Progress

| Milestone | Feature | Status |
|---|---|---|
| M1 | Core program + farmer registration + basic loan lifecycle | ✅ Complete |
| M2 | Loan disbursement + USDC vault initialization | ✅ Complete |
| M3 | Wallet connection + chain status + AdminConfig PDA | ✅ Complete |
| M4 | Live on-chain data — real farmer count, loan totals, risk scores | ✅ Complete |
| M5 | Full bankrun test suite — 33 tests passing | ✅ Complete |

---

## 📋 On-Chain Instructions

| Instruction | Description |
|---|---|
| `register_farmer` | Creates PDA per farmer with GPS coordinates |
| `declare_assets` | Farmer submits land size + livestock via USSD |
| `verify_farmer` | Field agent writes verified data + GPS boundary hash |
| `tag_livestock` | Unique QR/RFID tag per animal — prevents fraud |
| `update_risk_score` | AI oracle writes score (0–100) on-chain |
| `apply_for_loan` | Enforces 50% LTV collateral limit automatically |
| `approve_loan` | Lender sets interest rate and approves |
| `disburse_loan` | USDC vault disbursement to farmer wallet |
| `record_repayment` | Tracks payments, boosts score on-time repayment |
| `flag_default` | Marks default, penalises risk score by 15 points |
| `initialize_vault` | Sets up USDC vault for loan capital |

---

## 🔐 Verification Architecture

VeriFarm operates as a **distributed oracle network** with 
on-the-ground human nodes:

| Layer | Method | Anti-Fraud Measure |
|---|---|---|
| Land | GPS boundary polygon → SHA256 hash on-chain | Prevents double land claims |
| Livestock | QR/RFID tag → unique PDA per animal | Prevents duplicate registration |
| Human | Field agent physical validation | Immutable verification trail |
| Collateral | 50% LTV limit | Reflects agricultural asset illiquidity |

_Land boundary coordinates are hashed before on-chain storage and are only accessible to program-gated MFI PDAs. Raw GPS data is never publicly readable. Farmer consent is logged on-chain during USSD registration in compliance with Kenya's Data Protection Act 2019 and Tanzania's PDPA framework._
---

## 🤖 AI Risk Scoring

Scores are written on-chain by an oracle and are fully 
explainable — not a black box:

```
Score breakdown example:
📐 Farm Size (5 acres verified)      +24 pts  ✅
💳 Repayment History (first loan)    +15 pts  ➖  
🐄 Livestock Health (verified)       +22 pts  ✅
🌧️ Rainfall Index (below average)   -11 pts  ❌
─────────────────────────────────────────────
Total Score: 72 / 100 — Medium Risk
Max Eligible Loan: TZS 180,000 at 50% LTV
```

---

## 🧪 Running Tests

```bash
# Clone the repo
git clone https://github.com/yvonne-byte-crypto/Verifarm-backend.git
cd Verifarm-backend

# Install dependencies
yarn install

# Run the full test suite
anchor test

# Expected output:
# 33 passing tests
```

---

## 🚀 Deploy to Devnet

```bash
# Set up wallet
solana-keygen new
solana config set --url devnet
solana airdrop 2

# Build and deploy
anchor build
anchor deploy --provider.cluster devnet
```

---

## 🏦 Capital Model

VeriFarm provides the **verification and scoring rails** — 
licensed microfinance institutions and SACCOs in Kenya and 
Tanzania provide the regulated capital. This means:

- VeriFarm never holds customer funds
- Regulatory compliance sits with licensed MFI partners
- DeFi liquidity can flow through compliant institutional layer
- Farmers get credit. MFIs get reach. VeriFarm gets the oracle fee.

---

## 🌱 Roadmap

- [x] Core Anchor program deployed on Devnet
- [x] USDC vault + loan disbursement
- [x] Wallet connection + chain status
- [x] Live on-chain data feeds
- [x] 33 bankrun tests passing
- [ ] Africa's Talking USSD integration
- [ ] Continuous livestock health oracle
- [ ] MFI partner API integration
- [ ] Mainnet launch

---

## 🛠️ Built With

- **Solana + Anchor 0.31.0** — blockchain layer
- **Rust** — smart contract language
- **Bankrun** — fast Solana test framework
- **solana.new + Claude Code** — agentic engineering
- **USDC** — loan disbursement currency

---

## 👩🏾‍💻 Builder

**Yvonne Yuvenali** — Tanzanian innovator building oracle 
infrastructure for agricultural credit in East Africa.

- 🌍 solana.new Founding Builder — Pass #0142
- 🏆 Top 500 Global Finalist, RISE for the World (2022)
- 🎓 African Leadership Academy — Humility Award
- 📚 BSc Business Studies with AI for Enterprise, TUS Ireland
- 💡 Founder — OptiGrow, Creative Minds and Talents,
  Endelea Connective

---

*Built for the Colosseum Hackathon 2026 — for the farmers 
back home in Tanzania who deserve access to the financial 
system they've always been excluded from.* 🌍🌾
