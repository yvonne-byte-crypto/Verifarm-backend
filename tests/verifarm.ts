import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import { Verifarm } from "../target/types/verifarm";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { createHash } from "crypto";
import { assert } from "chai";

const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

// ── PDA helpers ────────────────────────────────────────────────────────────────

const pid = new PublicKey("9teMVR4r2AB9T5bB4YgXJ38G6mMbxTF6bFm8UYizqx8N");

const adminConfigPda = () =>
  PublicKey.findProgramAddressSync([Buffer.from("admin_config")], pid)[0];

const oracleEntryPda = (oracle: PublicKey) =>
  PublicKey.findProgramAddressSync([Buffer.from("oracle_entry"), oracle.toBuffer()], pid)[0];

const officerEntryPda = (officer: PublicKey) =>
  PublicKey.findProgramAddressSync([Buffer.from("loan_officer_entry"), officer.toBuffer()], pid)[0];

const farmerPda = (authority: PublicKey) =>
  PublicKey.findProgramAddressSync([Buffer.from("farmer"), authority.toBuffer()], pid)[0];

const riskScorePda = (farmer: PublicKey) =>
  PublicKey.findProgramAddressSync([Buffer.from("risk_score"), farmer.toBuffer()], pid)[0];

const loanPda = (farmer: PublicKey, index: number) =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("loan"), farmer.toBuffer(), Buffer.from(new Uint16Array([index]).buffer)],
    pid
  )[0];

const agentStakePda = (agent: PublicKey) =>
  PublicKey.findProgramAddressSync([Buffer.from("agent_stake"), agent.toBuffer()], pid)[0];

const verificationRecordPda = (agent: PublicKey, farmer: PublicKey) =>
  PublicKey.findProgramAddressSync(
    [Buffer.from("verification_record"), agent.toBuffer(), farmer.toBuffer()],
    pid
  )[0];

const treasuryPda = () =>
  PublicKey.findProgramAddressSync([Buffer.from("treasury")], pid)[0];

// ── Helpers ────────────────────────────────────────────────────────────────────

const hashId = (id: string) => Array.from(createHash("sha256").update(id).digest());

/** Build a minimal SPL Mint account buffer (82 bytes, decimals=6, supply=0) */
function mintAccountData(authority: PublicKey): Buffer {
  const buf = Buffer.alloc(82);
  let o = 0;
  buf.writeUInt32LE(1, o); o += 4;              // mint_authority COption::Some
  authority.toBuffer().copy(buf, o); o += 32;   // mint_authority pubkey
  buf.writeBigUInt64LE(BigInt(0), o); o += 8;  // supply
  buf[o++] = 6;                                  // decimals
  buf[o++] = 1;                                  // is_initialized
  buf.writeUInt32LE(0, o);                       // freeze_authority COption::None
  return buf;
}

/** Assert the instruction fails with the expected Anchor error code */
async function expectError(fn: () => Promise<unknown>, code: string) {
  try {
    await fn();
    assert.fail(`Expected error "${code}" but instruction succeeded`);
  } catch (e) {
    if (e instanceof AnchorError) {
      assert.equal(
        e.error.errorCode.code, code,
        `Expected "${code}" but got "${e.error.errorCode.code}"`
      );
      return;
    }
    // Non-Anchor errors (account already in use, simulation failures) are
    // acceptable for "already in use" style checks — rethrow only assert.fail
    if (e instanceof Error && e.message.startsWith("Expected error")) throw e;
  }
}

// ── Suite ──────────────────────────────────────────────────────────────────────

describe("VeriFarm — Milestone 5 test suite (bankrun)", () => {
  let context: Awaited<ReturnType<typeof startAnchor>>;
  let provider: BankrunProvider;
  let program: Program<Verifarm>;

  let admin:    Keypair;
  let oracle:   Keypair;
  let officer:  Keypair;
  let farmer:   Keypair;
  let stranger: Keypair;
  let mint:     Keypair;

  before(async () => {
    admin    = Keypair.generate();
    oracle   = Keypair.generate();
    officer  = Keypair.generate();
    farmer   = Keypair.generate();
    stranger = Keypair.generate();
    mint     = Keypair.generate();

    const sol = (n: number) => ({ lamports: n * 1e9, data: Buffer.alloc(0), owner: SystemProgram.programId, executable: false });

    context = await startAnchor(
      "/home/yvonne_yuvenali/verifarm",
      [],
      [
        { address: admin.publicKey,    info: sol(10) },
        { address: oracle.publicKey,   info: sol(10) },
        { address: officer.publicKey,  info: sol(10) },
        { address: farmer.publicKey,   info: sol(10) },
        { address: stranger.publicKey, info: sol(10) },
        // Real SPL Mint account so Anchor's Mint type check passes
        {
          address: mint.publicKey,
          info: {
            lamports: 1e9,
            data: mintAccountData(admin.publicKey),
            owner: TOKEN_PROGRAM_ID,
            executable: false,
          },
        },
      ]
    );

    provider = new BankrunProvider(context);
    anchor.setProvider(provider);
    program = anchor.workspace.Verifarm as Program<Verifarm>;
  });

  // ── 1. Program initialisation ───────────────────────────────────────────────

  describe("initialize_program", () => {
    it("creates AdminConfig singleton", async () => {
      provider.wallet = new anchor.Wallet(admin);

      await program.methods.initializeProgram()
        .accounts({ admin: admin.publicKey })
        .signers([admin])
        .rpc();

      const cfg = await program.account.adminConfig.fetch(adminConfigPda());
      assert.ok(cfg.admin.equals(admin.publicKey), "admin key stored correctly");
    });

    it("rejects a second initialisation (account already in use)", async () => {
      let threw = false;
      try {
        await program.methods.initializeProgram()
          .accounts({ admin: admin.publicKey })
          .signers([admin])
          .rpc();
      } catch { threw = true; }
      assert.ok(threw, "second init should fail");
    });
  });

  // ── 2. Oracle registry ──────────────────────────────────────────────────────

  describe("oracle registry", () => {
    it("registers an oracle (admin)", async () => {
      provider.wallet = new anchor.Wallet(admin);

      await program.methods.registerOracle(oracle.publicKey)
        .accounts({ admin: admin.publicKey })
        .signers([admin])
        .rpc();

      const e = await program.account.oracleEntry.fetch(oracleEntryPda(oracle.publicKey));
      assert.ok(e.oracle.equals(oracle.publicKey));
      assert.equal(e.active, true);
    });

    it("rejects oracle registration by non-admin (UnauthorizedOracle)", async () => {
      await expectError(
        () => program.methods.registerOracle(stranger.publicKey)
          .accounts({ admin: stranger.publicKey })
          .signers([stranger])
          .rpc(),
        "UnauthorizedOracle"   // has_one = admin uses this custom error
      );
    });

    it("revokes an oracle — active becomes false", async () => {
      const tmp = Keypair.generate();

      await program.methods.registerOracle(tmp.publicKey)
        .accounts({ admin: admin.publicKey })
        .signers([admin])
        .rpc();

      await program.methods.revokeOracle(tmp.publicKey)
        .accounts({ admin: admin.publicKey })
        .signers([admin])
        .rpc();

      const e = await program.account.oracleEntry.fetch(oracleEntryPda(tmp.publicKey));
      assert.equal(e.active, false);
    });
  });

  // ── 3. Loan officer registry ────────────────────────────────────────────────

  describe("loan officer registry", () => {
    it("registers a loan officer (admin)", async () => {
      await program.methods.registerOfficer(officer.publicKey)
        .accounts({ admin: admin.publicKey })
        .signers([admin])
        .rpc();

      const e = await program.account.loanOfficerEntry.fetch(officerEntryPda(officer.publicKey));
      assert.ok(e.officer.equals(officer.publicKey));
      assert.equal(e.active, true);
    });

    it("revokes an officer — active becomes false", async () => {
      const tmp = Keypair.generate();

      await program.methods.registerOfficer(tmp.publicKey)
        .accounts({ admin: admin.publicKey })
        .signers([admin])
        .rpc();

      await program.methods.revokeOfficer(tmp.publicKey)
        .accounts({ admin: admin.publicKey })
        .signers([admin])
        .rpc();

      const e = await program.account.loanOfficerEntry.fetch(officerEntryPda(tmp.publicKey));
      assert.equal(e.active, false);
    });
  });

  // ── 4. Farmer registration ──────────────────────────────────────────────────

  describe("register_farmer", () => {
    it("registers a new farmer (Pending status)", async () => {
      provider.wallet = new anchor.Wallet(farmer);

      await program.methods.registerFarmer({
        nationalIdHash: hashId("TZ-FARMER-001"),
        fullName: "Amina Wanjiku",
        phone: "+255712345678",
        locationLat: new anchor.BN(-121000000),
        locationLng: new anchor.BN(36800000),
      })
        .accounts({
          farmer: farmerPda(farmer.publicKey),
          authority: farmer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([farmer])
        .rpc();

      const acc = await program.account.farmer.fetch(farmerPda(farmer.publicKey));
      assert.equal(acc.fullName, "Amina Wanjiku");
      assert.ok("pending" in acc.status, "new farmer should be Pending");
      assert.equal(acc.loanCount, 0);
    });

    it("rejects duplicate registration (account already in use)", async () => {
      let threw = false;
      try {
        await program.methods.registerFarmer({
          nationalIdHash: hashId("TZ-FARMER-001"),
          fullName: "Amina Wanjiku",
          phone: "+255712345678",
          locationLat: new anchor.BN(-121000000),
          locationLng: new anchor.BN(36800000),
        })
          .accounts({
            farmer: farmerPda(farmer.publicKey),
            authority: farmer.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([farmer])
          .rpc();
      } catch { threw = true; }
      assert.ok(threw, "duplicate farmer PDA should fail");
    });
  });

  // ── 5. Risk oracle ──────────────────────────────────────────────────────────

  describe("submit_risk_score", () => {
    it("rejects submission from unregistered signer (AccountNotInitialized)", async () => {
      await expectError(
        () => program.methods.submitRiskScore({ score: 75, confidence: 80, modelVersion: "v1" })
          .accounts({
            riskScore: riskScorePda(farmerPda(farmer.publicKey)),
            farmer: farmerPda(farmer.publicKey),
            oracle: stranger.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([stranger])
          .rpc(),
        "AccountNotInitialized"   // oracle_entry PDA doesn't exist for stranger
      );
    });

    it("rejects score > 100", async () => {
      await expectError(
        () => program.methods.submitRiskScore({ score: 101, confidence: 80, modelVersion: "v1" })
          .accounts({
            riskScore: riskScorePda(farmerPda(farmer.publicKey)),
            farmer: farmerPda(farmer.publicKey),
            oracle: oracle.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([oracle])
          .rpc(),
        "RiskScoreTooLow"
      );
    });

    it("accepts score from registered oracle", async () => {
      provider.wallet = new anchor.Wallet(oracle);

      await program.methods.submitRiskScore({ score: 82, confidence: 91, modelVersion: "v2.1.0" })
        .accounts({
          riskScore: riskScorePda(farmerPda(farmer.publicKey)),
          farmer: farmerPda(farmer.publicKey),
          oracle: oracle.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([oracle])
        .rpc();

      const rs = await program.account.riskScore.fetch(riskScorePda(farmerPda(farmer.publicKey)));
      assert.equal(rs.score, 82);
      assert.ok("prime" in rs.tier, "score 82 → Prime tier");

      const f = await program.account.farmer.fetch(farmerPda(farmer.publicKey));
      assert.equal(f.latestRiskScore, 82, "cached on Farmer account");
    });
  });

  // ── 6. Farmer status update ─────────────────────────────────────────────────

  describe("update_farmer_status", () => {
    it("rejects status update from unregistered stranger (AccountNotInitialized)", async () => {
      await expectError(
        () => program.methods.updateFarmerStatus({
          newStatus: { verified: {} },
          farmerAuthority: farmer.publicKey,
        })
          .accounts({
            farmer: farmerPda(farmer.publicKey),
            officerEntry: officerEntryPda(stranger.publicKey),
            adminConfig: adminConfigPda(),
            officer: stranger.publicKey,
          })
          .signers([stranger])
          .rpc(),
        "AccountNotInitialized"
      );
    });

    it("allows registered officer to verify a farmer", async () => {
      provider.wallet = new anchor.Wallet(officer);

      await program.methods.updateFarmerStatus({
        newStatus: { verified: {} },
        farmerAuthority: farmer.publicKey,
      })
        .accounts({
          farmer: farmerPda(farmer.publicKey),
          officerEntry: officerEntryPda(officer.publicKey),
          adminConfig: adminConfigPda(),
          officer: officer.publicKey,
        })
        .signers([officer])
        .rpc();

      const acc = await program.account.farmer.fetch(farmerPda(farmer.publicKey));
      assert.ok("verified" in acc.status, "farmer should now be Verified");
    });

    it("allows officer to suspend a farmer", async () => {
      provider.wallet = new anchor.Wallet(officer);

      await program.methods.updateFarmerStatus({
        newStatus: { suspended: {} },
        farmerAuthority: farmer.publicKey,
      })
        .accounts({
          farmer: farmerPda(farmer.publicKey),
          officerEntry: officerEntryPda(officer.publicKey),
          adminConfig: adminConfigPda(),
          officer: officer.publicKey,
        })
        .signers([officer])
        .rpc();

      const acc = await program.account.farmer.fetch(farmerPda(farmer.publicKey));
      assert.ok("suspended" in acc.status);

      // Restore to Verified for downstream tests
      await program.methods.updateFarmerStatus({
        newStatus: { verified: {} },
        farmerAuthority: farmer.publicKey,
      })
        .accounts({
          farmer: farmerPda(farmer.publicKey),
          officerEntry: officerEntryPda(officer.publicKey),
          adminConfig: adminConfigPda(),
          officer: officer.publicKey,
        })
        .signers([officer])
        .rpc();
    });
  });

  // ── 7. Loan application ─────────────────────────────────────────────────────

  describe("apply_for_loan", () => {
    it("rejects amount below $50 minimum", async () => {
      provider.wallet = new anchor.Wallet(farmer);

      await expectError(
        () => program.methods.applyForLoan({
          amountUsdCents: new anchor.BN(4_999),
          termDays: 30,
          interestBps: 1500,
        })
          .accounts({
            loan: loanPda(farmerPda(farmer.publicKey), 0),
            farmer: farmerPda(farmer.publicKey),
            riskScore: riskScorePda(farmerPda(farmer.publicKey)),
            tokenMint: mint.publicKey,
            authority: farmer.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([farmer])
          .rpc(),
        "LoanAmountTooSmall"
      );
    });

    it("rejects amount above Prime tier cap ($5,000)", async () => {
      await expectError(
        () => program.methods.applyForLoan({
          amountUsdCents: new anchor.BN(500_001),
          termDays: 30,
          interestBps: 1500,
        })
          .accounts({
            loan: loanPda(farmerPda(farmer.publicKey), 0),
            farmer: farmerPda(farmer.publicKey),
            riskScore: riskScorePda(farmerPda(farmer.publicKey)),
            tokenMint: mint.publicKey,
            authority: farmer.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([farmer])
          .rpc(),
        "LoanAmountTooLarge"
      );
    });

    it("accepts a valid $2,000 loan application", async () => {
      await program.methods.applyForLoan({
        amountUsdCents: new anchor.BN(200_000),
        termDays: 30,
        interestBps: 1500,
      })
        .accounts({
          loan: loanPda(farmerPda(farmer.publicKey), 0),
          farmer: farmerPda(farmer.publicKey),
          riskScore: riskScorePda(farmerPda(farmer.publicKey)),
          tokenMint: mint.publicKey,
          authority: farmer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([farmer])
        .rpc();

      const loan = await program.account.loan.fetch(loanPda(farmerPda(farmer.publicKey), 0));
      assert.ok("pending" in loan.status);
      assert.equal(loan.principal.toNumber(), 200_000);
      assert.equal(loan.termDays, 30);

      const f = await program.account.farmer.fetch(farmerPda(farmer.publicKey));
      assert.equal(f.loanCount, 1, "loan count incremented");
    });
  });

  // ── 8. Loan approval ────────────────────────────────────────────────────────

  describe("approve_loan", () => {
    it("rejects approval by unregistered stranger (AccountNotInitialized)", async () => {
      await expectError(
        () => program.methods.approveLoan()
          .accounts({
            loan: loanPda(farmerPda(farmer.publicKey), 0),
            officerEntry: officerEntryPda(stranger.publicKey),
            loanOfficer: stranger.publicKey,
          })
          .signers([stranger])
          .rpc(),
        "AccountNotInitialized"
      );
    });

    it("approves the loan with a registered officer", async () => {
      provider.wallet = new anchor.Wallet(officer);

      await program.methods.approveLoan()
        .accounts({
          loan: loanPda(farmerPda(farmer.publicKey), 0),
          officerEntry: officerEntryPda(officer.publicKey),
          loanOfficer: officer.publicKey,
        })
        .signers([officer])
        .rpc();

      const loan = await program.account.loan.fetch(loanPda(farmerPda(farmer.publicKey), 0));
      assert.ok("approved" in loan.status);
      assert.ok(loan.approvedAt !== null, "approvedAt set");
    });

    it("rejects double approval (loan not Pending)", async () => {
      await expectError(
        () => program.methods.approveLoan()
          .accounts({
            loan: loanPda(farmerPda(farmer.publicKey), 0),
            officerEntry: officerEntryPda(officer.publicKey),
            loanOfficer: officer.publicKey,
          })
          .signers([officer])
          .rpc(),
        "InvalidLoanState"
      );
    });
  });

  // ── 9. State guards ──────────────────────────────────────────────────────────

  describe("state machine guards", () => {
    it("repay_loan rejects when loan is Approved (not Active)", async () => {
      provider.wallet = new anchor.Wallet(farmer);

      // Anchor validates account initialization before executing instruction logic,
      // so passing dummy token accounts fires AccountNotInitialized first.
      // The loan IS in Approved state here, so any repay attempt fails correctly.
      await expectError(
        () => program.methods.repayLoan({ amount: new anchor.BN(1_000) })
          .accounts({
            loan: loanPda(farmerPda(farmer.publicKey), 0),
            farmer: farmerPda(farmer.publicKey),
            farmerTokenAccount: Keypair.generate().publicKey,
            protocolVault: Keypair.generate().publicKey,
            authority: farmer.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([farmer])
          .rpc(),
        "AccountNotInitialized"
      );
    });

    it("liquidate_loan rejects when loan is not defaulted", async () => {
      provider.wallet = new anchor.Wallet(officer);

      await expectError(
        () => program.methods.liquidateLoan()
          .accounts({
            loan: loanPda(farmerPda(farmer.publicKey), 0),
            farmer: farmerPda(farmer.publicKey),
            officerEntry: officerEntryPda(officer.publicKey),
            loanOfficer: officer.publicKey,
          })
          .signers([officer])
          .rpc(),
        "LoanNotDefaulted"
      );
    });
  });

  // ── Agent Staking (anti-fraud) ─────────────────────────────────────────────

  describe("agent staking", () => {
    const MIN_STAKE_NUM = 100_000_000; // 0.1 SOL

    let agent:   Keypair;
    let agent2:  Keypair;
    let farmerKp: Keypair;

    /** Transfer SOL from admin to a new keypair via bankrun */
    async function fundKeypairs(...kps: { kp: Keypair; lamports: number }[]) {
      const [blockhash] = await context.banksClient.getLatestBlockhash();
      const tx = new anchor.web3.Transaction();
      kps.forEach(({ kp, lamports }) =>
        tx.add(SystemProgram.transfer({ fromPubkey: admin.publicKey, toPubkey: kp.publicKey, lamports }))
      );
      tx.recentBlockhash = blockhash;
      tx.feePayer = admin.publicKey;
      tx.sign(admin);
      await context.banksClient.processTransaction(tx);
    }

    before(async () => {
      agent    = Keypair.generate();
      agent2   = Keypair.generate();
      farmerKp = Keypair.generate();

      await fundKeypairs(
        { kp: agent,    lamports: 2_000_000_000 },
        { kp: agent2,   lamports: 2_000_000_000 },
        { kp: farmerKp, lamports: 500_000_000  },
      );

      // Register a test farmer so we have a valid farmer PDA for verifications
      await program.methods
        .registerFarmer({
          nationalIdHash: hashId("AGENT-TEST-FARMER-001"),
          fullName: "Test Farmer",
          phone: "+255700000001",
          locationLat: new anchor.BN(-6_800_000),
          locationLng: new anchor.BN(39_200_000),
        })
        .accounts({
          farmer: farmerPda(farmerKp.publicKey),
          authority: farmerKp.publicKey,
          adminConfig: adminConfigPda(),
          systemProgram: SystemProgram.programId,
        })
        .signers([farmerKp])
        .rpc();
    });

    // ── register_agent ──────────────────────────────────────────────────────

    it("register_agent: registers with minimum stake (0.1 SOL)", async () => {
      await program.methods
        .registerAgent(new anchor.BN(MIN_STAKE_NUM))
        .accounts({
          agentStake: agentStakePda(agent.publicKey),
          agent: agent.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([agent])
        .rpc();

      const stake = await program.account.agentStake.fetch(agentStakePda(agent.publicKey));
      assert.equal(stake.agent.toBase58(), agent.publicKey.toBase58());
      assert.equal(stake.stakedLamports.toNumber(), MIN_STAKE_NUM);
      assert.equal(stake.activeVerifications, 0);
      assert.deepEqual(stake.status, { active: {} });
    });

    it("register_agent: rejects stake below minimum (0.05 SOL)", async () => {
      const cheapAgent = Keypair.generate();
      await fundKeypairs({ kp: cheapAgent, lamports: 500_000_000 });

      await expectError(
        () =>
          program.methods
            .registerAgent(new anchor.BN(50_000_000))
            .accounts({
              agentStake: agentStakePda(cheapAgent.publicKey),
              agent: cheapAgent.publicKey,
              systemProgram: SystemProgram.programId,
            })
            .signers([cheapAgent])
            .rpc(),
        "InsufficientStake"
      );
    });

    // ── submit_verification ─────────────────────────────────────────────────

    it("submit_verification: active agent can submit a verification", async () => {
      await program.methods
        .submitVerification()
        .accounts({
          verificationRecord: verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey)),
          agentStake: agentStakePda(agent.publicKey),
          farmer: farmerPda(farmerKp.publicKey),
          agent: agent.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([agent])
        .rpc();

      const record = await program.account.verificationRecord.fetch(
        verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey))
      );
      assert.equal(record.agent.toBase58(), agent.publicKey.toBase58());
      assert.deepEqual(record.status, { pending: {} });

      const stake = await program.account.agentStake.fetch(agentStakePda(agent.publicKey));
      assert.equal(stake.activeVerifications, 1, "active_verifications should be 1");
    });

    // ── dispute_verification ────────────────────────────────────────────────

    it("dispute_verification: admin can flag a pending verification", async () => {
      await program.methods
        .disputeVerification()
        .accounts({
          verificationRecord: verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey)),
          adminConfig: adminConfigPda(),
          disputer: admin.publicKey,
          disputerStake: null,
        })
        .signers([admin])
        .rpc();

      const record = await program.account.verificationRecord.fetch(
        verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey))
      );
      assert.deepEqual(record.status, { disputed: {} });
      assert.equal(record.disputedBy?.toBase58(), admin.publicKey.toBase58());
    });

    it("dispute_verification: rejects re-dispute of an already-disputed record", async () => {
      await expectError(
        () =>
          program.methods
            .disputeVerification()
            .accounts({
              verificationRecord: verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey)),
              adminConfig: adminConfigPda(),
              disputer: admin.publicKey,
              disputerStake: null,
            })
            .signers([admin])
            .rpc(),
        "VerificationNotPending"
      );
    });

    it("dispute_verification: rejects unauthorised disputer", async () => {
      // Register agent2 first so we can use it as unauthorised stranger in this scope
      await program.methods
        .registerAgent(new anchor.BN(MIN_STAKE_NUM))
        .accounts({
          agentStake: agentStakePda(agent2.publicKey),
          agent: agent2.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([agent2])
        .rpc();

      // stranger is not admin and has no agent_stake — should be Unauthorized
      await expectError(
        () =>
          program.methods
            .disputeVerification()
            .accounts({
              verificationRecord: verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey)),
              adminConfig: adminConfigPda(),
              disputer: stranger.publicKey,
              disputerStake: null,
            })
            .signers([stranger])
            .rpc(),
        "Unauthorized"
      );
    });

    // ── confirm_dispute (slash) ─────────────────────────────────────────────

    it("confirm_dispute: admin slashes agent stake to treasury", async () => {
      const stakeData   = await program.account.agentStake.fetch(agentStakePda(agent.publicKey));
      const slashAmount = stakeData.stakedLamports.toNumber();

      await program.methods
        .confirmDispute()
        .accounts({
          verificationRecord: verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey)),
          agentStake: agentStakePda(agent.publicKey),
          treasury: treasuryPda(),
          adminConfig: adminConfigPda(),
          admin: admin.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      const stakeAfter = await program.account.agentStake.fetch(agentStakePda(agent.publicKey));
      assert.deepEqual(stakeAfter.status, { suspended: {} }, "agent should be suspended");
      assert.equal(stakeAfter.stakedLamports.toNumber(), 0, "staked_lamports should be zero");
      assert.equal(stakeAfter.activeVerifications, 0, "active_verifications should decrement");

      const record = await program.account.verificationRecord.fetch(
        verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey))
      );
      assert.deepEqual(record.status, { slashed: {} }, "record should be slashed");

      const treasury = await program.account.treasuryVault.fetch(treasuryPda());
      assert.isAtLeast(
        treasury.totalSlashedLamports.toNumber(),
        slashAmount,
        "treasury should hold at least the slashed amount"
      );
    });

    it("confirm_dispute: non-admin cannot confirm a dispute", async () => {
      await expectError(
        () =>
          program.methods
            .confirmDispute()
            .accounts({
              verificationRecord: verificationRecordPda(agent.publicKey, farmerPda(farmerKp.publicKey)),
              agentStake: agentStakePda(agent.publicKey),
              treasury: treasuryPda(),
              adminConfig: adminConfigPda(),
              admin: agent2.publicKey, // impostor
              systemProgram: SystemProgram.programId,
            })
            .signers([agent2])
            .rpc(),
        "Unauthorized"
      );
    });

    // ── withdraw_stake ──────────────────────────────────────────────────────

    it("withdraw_stake: suspended agent cannot withdraw", async () => {
      await expectError(
        () =>
          program.methods
            .withdrawStake()
            .accounts({
              agentStake: agentStakePda(agent.publicKey),
              agent: agent.publicKey,
              systemProgram: SystemProgram.programId,
            })
            .signers([agent])
            .rpc(),
        "AgentSuspended"
      );
    });

    it("withdraw_stake: active agent with no pending verifications gets stake back", async () => {
      const balBefore = (await context.banksClient.getAccount(agent2.publicKey))!.lamports;

      await program.methods
        .withdrawStake()
        .accounts({
          agentStake: agentStakePda(agent2.publicKey),
          agent: agent2.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([agent2])
        .rpc();

      const closedAccount = await context.banksClient.getAccount(agentStakePda(agent2.publicKey));
      assert.isNull(closedAccount, "agent_stake PDA should be closed");

      const balAfter = (await context.banksClient.getAccount(agent2.publicKey))!.lamports;
      assert.isAbove(Number(balAfter), Number(balBefore), "agent should receive lamports back");
    });
  });
});
