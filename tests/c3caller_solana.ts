import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { C3callerSolana } from "../target/types/c3caller_solana";

describe("c3caller_solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.C3callerSolana as Program<C3callerSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
