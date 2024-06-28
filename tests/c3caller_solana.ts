import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { C3callerSolana } from "../target/types/c3caller_solana";

describe("c3caller_solana", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const systemProgram = anchor.web3.SystemProgram;
  const program = anchor.workspace.C3callerSolana as Program<C3callerSolana>;

  const C3UUID_KEEPER_SEED = Buffer.from("c3uuidseed");
  const PAUSE_SEED = Buffer.from("pauseseed");
  const [c3uuidkeeper,_bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [C3UUID_KEEPER_SEED],
    program.programId
  );


  const[pause, _bumps] = anchor.web3.PublicKey.findProgramAddressSync([PAUSE_SEED], program.programId)

  it("Is initialized!", async () => {
    
 
    const tx = await program.methods.initialize().accounts({ pause:pause, c3Uuid:c3uuidkeeper,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should successfully call c3call with valid inputs", async()=>{

    const dappId = new anchor.BN(1);
    const caller = provider.publicKey;
    const to = '9Jt8mC9HXvh2g5s3PbTsNU71RS9MXUbhEMEmLTixYirb';
    const toChainId = 'chain1';
    const data = Buffer.from('some_data');
    const extra = Buffer.from('some_extra_data');
    const listenerLogC3Call = program.addEventListener('LogC3Call', (event, slot) => {
      console.log(`slot ${slot} event value ${JSON.stringify(event)} `);
    });

    const tx = await program.methods.c3Call(dappId, caller, to, toChainId, data, extra).accounts({
      pause:pause, c3Uuid:c3uuidkeeper
    }).rpc()

    await new Promise((resolve) => setTimeout(resolve, 5000));
    program.removeEventListener(listenerLogC3Call);
     console.log("Your transaction signature",tx);
   });

   it("Should successfully call broadcast with valid inputs", async()=>{

    const dappId = new anchor.BN(1);
    const caller = provider.publicKey;
    const to = ['9Jt8mC9HXvh2g5s3PbTsNU71RS9MXUbhEMEmLTixYirb','8Jt8mC9HXvh2g5s3PbTsNs71RS9MXUbhEMEmLTixYirv'];
    const toChainId = ['chain1','chain2'];
    const data = Buffer.from('some_data');
    const listenerLogC3Call = program.addEventListener('LogC3Call', (event, slot) => {
      console.log(`slot ${slot} event value ${JSON.stringify(event)} `);
    });

    const tx = await program.methods.c3Broadcast(dappId, caller, to, toChainId, data).accounts({
      pause:pause, c3Uuid:c3uuidkeeper
    }).rpc()

    await new Promise((resolve) => setTimeout(resolve, 5000));
    program.removeEventListener(listenerLogC3Call);
     console.log("Your transaction signature",tx);
   });



});
