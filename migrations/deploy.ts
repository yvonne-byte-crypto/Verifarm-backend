import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);
  // Post-deploy initialization (e.g., create admin PDA, seed oracle registry)
  console.log("VeriFarm deployed. Next: initialize admin PDA and register oracle keypair.");
};
