import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { C3callerSolana } from "../target/types/c3caller_solana";

describe("c3caller_solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const systemProgram = anchor.web3.SystemProgram;
  const program = anchor.workspace.C3callerSolana as Program<C3callerSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const seeds = []
    const [c3uuidkeeper,_bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

   // const[pause, _bumps] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId)
    const pause = anchor.web3.Keypair.generate()
    const tx = await program.methods.initialize().accounts({ pause:pause.publicKey, c3Uuid:c3uuidkeeper,
      signer:provider.wallet.publicKey,
      systemProgram:systemProgram.programId
    }).signers([pause])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  // it("call c3call", async()=>{


  // //   const tx = await program.methods.c3call().rpc();
  // //   console.log("Your transaction signature",tx);
  //  });
});
