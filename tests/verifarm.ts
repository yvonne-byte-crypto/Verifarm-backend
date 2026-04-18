import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Verifarm } from "../target/types/verifarm";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { createHash } from "crypto";
import { assert } from "chai";

describe("verifarm", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Verifarm as Program<Verifarm>;

  // Test keypairs
  const farmer = provider.wallet;
  const fieldOfficer = Keypair.generate();
  const loanOfficer = Keypair.generate();
  const oracle = Keypair.generate();

  // PDA helpers
  const farmerPda = (authority: PublicKey) =>
    PublicKey.findProgramAddressSync(
      [Buffer.from("farmer"), authority.toBuffer()],
      program.programId
    );

  const riskScorePda = (farmerPda: PublicKey) =>
    PublicKey.findProgramAddressSync(
      [Buffer.from("risk_score"), farmerPda.toBuffer()],
      program.programId
    );

  const loanPda = (farmerPda: PublicKey, loanIndex: number) =>
    PublicKey.findProgramAddressSync(
      [
        Buffer.from("loan"),
        farmerPda.toBuffer(),
        Buffer.from(new Uint16Array([loanIndex]).buffer),
      ],
      program.programId
    );

  const assetPda = (farmerPda: PublicKey, assetIndex: number) =>
    PublicKey.findProgramAddressSync(
      [
        Buffer.from("asset"),
        farmerPda.toBuffer(),
        Buffer.from(new Uint16Array([assetIndex]).buffer),
      ],
      program.programId
    );

  before(async () => {
    // Airdrop to field officer, loan officer, oracle for tx fees
    await Promise.all([
      provider.connection.requestAirdrop(fieldOfficer.publicKey, 2e9),
      provider.connection.requestAirdrop(loanOfficer.publicKey, 2e9),
      provider.connection.requestAirdrop(oracle.publicKey, 2e9),
    ]);
    await new Promise((r) => setTimeout(r, 1000));
  });

  it("registers a farmer", async () => {
    const [farmerAccount] = farmerPda(farmer.publicKey);
    const nationalIdHash = createHash("sha256")
      .update("KE-12345678")
      .digest();

    await program.methods
      .registerFarmer({
        nationalIdHash: Array.from(nationalIdHash),
        fullName: "Amina Wanjiku",
        phone: "+254712345678",
        locationLat: BigInt(-121000000), // -1.21° (Nairobi region)
        locationLng: BigInt(36800000),   //  36.8°
      })
      .accounts({
        farmer: farmerAccount,
        authority: farmer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.farmer.fetch(farmerAccount);
    assert.equal(account.fullName, "Amina Wanjiku");
    assert.equal(account.status.pending !== undefined, true);
    assert.equal(account.loanCount, 0);
  });

  it("submits an AI risk score", async () => {
    const [farmerAccount] = farmerPda(farmer.publicKey);
    const [riskScoreAccount] = riskScorePda(farmerAccount);

    await program.methods
      .submitRiskScore({
        score: 75,
        confidence: 88,
        modelVersion: "v2.1.0",
      })
      .accounts({
        riskScore: riskScoreAccount,
        farmer: farmerAccount,
        oracle: oracle.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([oracle])
      .rpc();

    const account = await program.account.riskScore.fetch(riskScoreAccount);
    assert.equal(account.score, 75);
    assert.equal(account.tier.standard !== undefined, true);
  });

  it("verifies a land asset", async () => {
    const [farmerAccount] = farmerPda(farmer.publicKey);
    const [assetAccount] = assetPda(farmerAccount, 0);

    await program.methods
      .verifyAsset({
        assetType: { land: {} },
        description: "2-acre maize farm, Kiambu County",
        valueUsdCents: 80000, // $800
        assetIndex: 0,
      })
      .accounts({
        asset: assetAccount,
        farmer: farmerAccount,
        fieldOfficer: fieldOfficer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([fieldOfficer])
      .rpc();

    const account = await program.account.asset.fetch(assetAccount);
    assert.equal(account.verified, true);
    assert.equal(account.valueUsdCents.toNumber(), 80000);
  });

  it("applies for a loan", async () => {
    // NOTE: farmer must be Verified status — update via admin instruction first
    // This test will fail until FarmerStatus::Verified is set
    const [farmerAccount] = farmerPda(farmer.publicKey);
    const [riskScoreAccount] = riskScorePda(farmerAccount);
    const [loanAccount] = loanPda(farmerAccount, 0);

    // Skipping: farmer is Pending, not Verified. Wire up admin/officer verification first.
    console.log(
      "Loan apply test: farmer must be Verified — implement update_status instruction"
    );
  });

  it("full lifecycle: register → score → verify asset → apply → approve → repay", async () => {
    // End-to-end happy path — implement once token vault is wired up
    console.log("E2E lifecycle test: stub — implement with USDC token accounts");
  });
});
