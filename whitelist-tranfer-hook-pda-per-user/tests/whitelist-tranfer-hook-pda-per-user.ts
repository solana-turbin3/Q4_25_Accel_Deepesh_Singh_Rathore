import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WhitelistTranferHookPdaPerUser } from "../target/types/whitelist_tranfer_hook_pda_per_user";

describe("whitelist-tranfer-hook-pda-per-user", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const wallet = provider.wallet;
  const program = anchor.workspace.whitelistTranferHookPdaPerUser as Program<WhitelistTranferHookPdaPerUser>;

  it("Is initialized!", async () => {
    const target1 = anchor.web3.Keypair.generate();

    const tx = await program.methods.initialize().accounts({
      admin : wallet.publicKey,
      targetAddress : target1.publicKey
    }).rpc();

    const [whitelistPDATarget1,_] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("whitelist"),target1.publicKey.toBuffer()],
      program.programId
    )

    console.log("Your transaction signature", tx);
  });
});
