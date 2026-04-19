import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Verifarm } from "../target/types/verifarm";
import { PublicKey, SystemProgram } from "@solana/web3.js";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Verifarm as Program<Verifarm>;

  const [adminConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from("admin_config")],
    program.programId
  );

  console.log("Program ID :", program.programId.toString());
  console.log("Admin      :", provider.wallet.publicKey.toString());
  console.log("AdminConfig:", adminConfig.toString());

  // Check if already initialized
  try {
    const existing = await program.account.adminConfig.fetch(adminConfig);
    console.log("✓ AdminConfig already initialized at", adminConfig.toString());
    console.log("  admin:", existing.admin.toString());
    return;
  } catch {
    // Not yet initialized — proceed
  }

  const tx = await program.methods
    .initializeProgram()
    .accounts({
      admin: provider.wallet.publicKey,
    })
    .rpc();

  console.log("✓ initialize_program tx:", tx);

  const account = await program.account.adminConfig.fetch(adminConfig);
  console.log("✓ AdminConfig created");
  console.log("  admin         :", account.admin.toString());
  console.log("  initialized_at:", account.initializedAt.toString());
}

main().catch((e) => { console.error(e); process.exit(1); });
