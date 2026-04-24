# 🌾 VeriFarm — Solana Anchor Backend

> DePIN oracle network for physical agricultural asset
> verification on Solana — field nodes read farms,
> Solana records the truth, DeFi lends with confidence.

![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?style=flat&logo=solana)
![Anchor](https://img.shields.io/badge/Anchor-0.31.0-blue)
![Tests](https://img.shields.io/badge/Tests-37%20Passing-brightgreen)
![Status](https://img.shields.io/badge/Status-Live%20on%20Devnet-brightgreen)
![Built With](https://img.shields.io/badge/Built%20with-Claude%20Code%20%2B%20solana.new-orange)

---

## ⛓️ Deployed Program

**Program ID:** `9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N`
**Network:** Devnet
**Framework:** Anchor 0.31.0
**Tests:** 37 bankrun tests passing

🔗 [View on Solana Explorer](https://explorer.solana.com/address/9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N?cluster=devnet)
🖥️ [Frontend Dashboard](https://verifarm-frontend.vercel.app)
📁 [Frontend Repo](https://github.com/yvonne-byte-crypto/verifarm-frontend)
📱 [USSD Server](https://github.com/yvonne-byte-crypto/verifarm-ussd)

---

## 🏗️ Milestone Progress

| Milestone | Feature | Status |
|---|---|---|
| M1 | Core program + farmer registration + basic loan lifecycle | ✅ Complete |
| M2 | Loan disbursement + USDC vault initialization | ✅ Complete |
| M3 | Wallet connection + chain status + AdminConfig PDA | ✅ Complete |
| M4 | Live on-chain data feeds | ✅ Complete |
| M5 | Full bankrun test suite — 37 tests passing | ✅ Complete |
| M6 | Agent staking anti-fraud layer | ✅ Complete |

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
| `register_agent` | Agent stakes SOL to become verified oracle node |
| `dispute_verification` | Flags a verification as potentially fraudulent |
| `confirm_dispute` | Admin confirms fraud — triggers stake slash |
| `withdraw_stake` | Agent in good standing withdraws their stake |

---

## 🛡️ Anti-Fraud: Agent Staking

Field agents must stake SOL on-chain before submitting
any verification — creating real economic accountability:

```
Agent registers → Stakes SOL → Submits verification
                                      ↓
                              Dispute window opens (72hrs)
                                      ↓
                    No dispute → Verification confirmed ✅
                    Dispute raised → Admin reviews
                                      ↓
                              Fraud confirmed → Stake slashed
                              Agent suspended permanently
```
World ID IDKit widget integrated for agent registration 
UX — server-side proof verification scoped for 
production deployment

---

## 🔐 Verification Architecture

| Layer | Method | Anti-Fraud Measure |
|---|---|---|
| Land | GPS polygon → SHA256 hash on-chain | Prevents double claims |
| Livestock | QR/RFID → unique PDA per animal | Prevents duplication |
| Agent | SOL stake + cryptographic signature | Economic deterrent |
| Staleness | last_verified_at TTL — 180 days | Prevents stale data |
| Collateral | 50% LTV limit | Reflects illiquidity risk |

---

## 🤖 AI Risk Scoring — Explainable

```
Score breakdown example:
📐 Farm Size (5 acres verified)      +28 pts  ✅
🐄 Livestock Health (verified)       +32 pts  ✅
💳 Repayment History (first loan)    ±0  pts  ➖
🌧️ Rainfall Index (below average)   -8  pts  ❌
─────────────────────────────────────────────
Total Score: 85 / 100 — Low Risk
Max Eligible Loan: TZS 420,000 at 50% LTV
```
Scores are decomposed into four weighted factors: GPS 
polygon quality, attestation age, agent stake history, 
and M-Pesa repayment record — displayed as a visual 
breakdown per borrower profile on the lender dashboard.

---

## 🛡️ Threat Model

**Colluding Agent Ring Defense:**
VeriFarm requires a minimum of 2 independent agents from
different geographic zones to attest the same boundary
before it becomes eligible for loan scoring. Agents cannot
attest boundaries in zones where they are registered as
landowners. Combined with SOL staking and slashing,
coordinated fraud is economically irrational — the cost
of staking across multiple agents exceeds the fraudulent
loan value at our 50% LTV limit.

**GPS Spoofing Defense:**
Cross-validation between independent agents in different
geographic zones makes coordinate spoofing detectable.
Staleness TTL ensures boundary attestations older than
180 days require re-verification before scoring.

**Data Privacy:**
All sensitive farmer data is hashed on-chain. Raw data
is only accessible to authorized MFI PDAs through
program-gated instructions.

**Community Collusion Defense:**
World ID prevents sybil attacks — one human, one agent 
account. To prevent self-dealing between legitimate agents 
in the same community, VeriFarm enforces geographic 
independence: cross-attesting agents must operate from GPS 
coordinates more than 50km from the primary agent's 
registered location. Any agent can challenge a boundary 
attestation and earn the slashed stake if the dispute is 
upheld by a threshold of geographically distant observers.
This makes community-level collusion economically irrational 
even among World ID verified agents.

---

## 🔐 Privacy & Compliance

Land boundary coordinates are hashed before on-chain
storage and are only accessible to program-gated MFI PDAs.
Raw GPS data is never publicly readable.

Farmer consent is logged on-chain during USSD registration
in compliance with Kenya's Data Protection Act 2019 and
Tanzania's PDPA framework.

MFI partners access full off-chain KYC data through their
existing Customer Due Diligence processes. On-chain stores
only the scored hash — fully compatible with CBK and BoT
AML requirements.

---

## 💰 Capital Model

VeriFarm provides verification and scoring rails —
licensed MFIs and SACCOs provide regulated capital.
CASH (Phantom Frontier) is used for loan disbursement. 
The lender dashboard allows MFI partners to configure 
an alternative stablecoin fallback (USDC, USDT) to 
mitigate single-stablecoin concentration risk.

- Currently in conversations with FINCA Tanzania,
  BRAC Tanzania, Equity Bank Tanzania, and NMB Bank
- Initial pilot target: Manyara, Tanzania
- VeriFarm earns oracle attestation fee per verified farmer
- Farmers always access VeriFarm for free
- 
---

## 🧪 Running Tests

```bash
git clone https://github.com/yvonne-byte-crypto/Verifarm-backend.git
cd Verifarm-backend
yarn install
anchor test

# Expected output:
# 37 passing tests
# 0 failing
```

---

## 🚀 Deploy to Devnet

```bash
solana-keygen new
solana config set --url devnet
solana airdrop 2
anchor build
anchor deploy --provider.cluster devnet
```

---

## 🌱 Roadmap

- [x] Core Anchor program deployed on Devnet
- [x] USDC vault + loan disbursement
- [x] Wallet connection + chain status
- [x] Live on-chain data feeds
- [x] 37 bankrun tests passing
- [x] Agent staking anti-fraud layer
- [x] Staleness TTL on boundary attestations
- [ ] Africa's Talking USSD production number
- [ ] Enhanced livestock attestation — mandatory photo 
  hash on-chain (Arweave) + higher stake requirement 
  (Phase 2)
- [ ] Continuous livestock health oracle
- [ ] MFI partner API integration
- [ ] Geographic diversity enforcement
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

**Yvonne Yuvenali** — Tanzanian innovator building DePIN
oracle infrastructure for agricultural credit in East Africa.

- 🌍 solana.new Founding Builder — Pass #0142
- 🏆 1st Place — Junior Achievement Africa Competition (2024)
- 🌍 Top 500 Global Finalist — RISE for the World (2023)
- 🎓 Valedictorian — 10 Million African Girls programme (2025)
- 💰 Ei Electronics Scholar — TUS Ireland
- 💡 Founder — OptiGrow, Creative Minds, Endelea Connective

---

*Built for the Colosseum Hackathon 2026 — for the farmers
back home in Tanzania who deserve access to the financial
system they've always been excluded from.* 🌍🌾
