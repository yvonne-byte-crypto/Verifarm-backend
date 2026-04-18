# VeriFarm — Solana Anchor Program

Smart contract backend for VeriFarm — agricultural lending 
platform for smallholder farmers in East Africa.

**Program ID:** `9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N`
**Network:** Devnet
**Framework:** Anchor 0.31.0

🔗 [View on Solana Explorer](https://explorer.solana.com/address/9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N?cluster=devnet)

🖥️ [Frontend Dashboard](https://verifarm-frontend.vercel.app)

## Instructions
- register_farmer — PDA per farmer with GPS
- declare_assets — land + livestock submission
- verify_farmer — field agent verification on-chain
- tag_livestock — unique QR/RFID per animal
- update_risk_score — AI oracle writes score
- apply_for_loan — 70% LTV enforced automatically
- approve_loan — lender approval
- disburse_loan — triggers disbursement
- record_repayment — tracks payments
- flag_default — penalises risk score

## Deploy
```bash
anchor build
anchor deploy --provider.cluster devnet
```

Built for Colosseum Hackathon 2026 🌍
