import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Theia } from "../target/types/theia";
import { TheiaUuidKeeper } from "../target/types/theia_uuid_keeper";
import { C3callerSolana } from "../target/types/c_3caller_solana";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("theia", () => {

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider)

    const systemProgram = anchor.web3.SystemProgram
    const theia = anchor.workspace.Theia as Program<Theia>
    const theiaUuidKeeper =  anchor.workspace.TheiaUuidKeeper as Program<TheiaUuidKeeper>
    const c_3caller_solana = anchor.workspace.C3callerSolana as Program<C3callerSolana>


    const C3UUID_KEEPER_SEED = Buffer.from("c3uuidseed");
    const PAUSE_SEED = Buffer.from("pauseseed");
    const OWENER_SEED = Buffer.from("ownerkeyseed")
    const CURRENT_NONCE_SEED = Buffer.from("current_nonce");
    const [c3uuidkeeper,] = anchor.web3.PublicKey.findProgramAddressSync([C3UUID_KEEPER_SEED],c_3caller_solana.programId );
    
    const[pause, _bumps] = anchor.web3.PublicKey.findProgramAddressSync([PAUSE_SEED], c_3caller_solana.programId)
    const[owner,]= anchor.web3.PublicKey.findProgramAddressSync([OWENER_SEED],c_3caller_solana.programId)

  
    const [currentNonce,] = anchor.web3.PublicKey.findProgramAddressSync([CURRENT_NONCE_SEED],theiaUuidKeeper.programId);

//const  currentNoncePda = await program.account.currentNonce.fetch(currentNonceAccount);
  //  const currentNonce = currentNoncePda.nonce;

  // 3. Calculate the PDA for the uuid_nonce account
   

    it("Is initialized!", async()=>{

        console.log("hello world")

        const nonce = await theiaUuidKeeper.account.currentNonce.fetch(currentNonce)
        const current_nonce = nonce.nonce.add(new anchor.BN(1))

        const [uuidNoncePDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("uuid_nonce"),
            Buffer.from(current_nonce.toArrayLike(Buffer, 'be', 8))
          ],
          theiaUuidKeeper.programId
        );

        console.log(`current nonce is ${current_nonce}`)
        console.log(`current nonce pda is ${uuidNoncePDA.toBase58()}`)

       let tx =  await theia.methods.theiaCrossEvm({
            target: "your_target_address", // Replace with actual target address
            receiver: "your_receiver_address", // Replace with actual receiver address
            amount: new anchor.BN(0), // Replace with actual amount
            feeAmount: new anchor.BN(0), // Replace with actual fee amount
            toChainId: new anchor.BN(0), // Replace with actual chain ID
            tokenId: "your_token_id", // Replace with actual token ID
            feeTokenId: "your_fee_token_id" // Replace with actual fee token ID
        }).accounts({
            currentNonce: currentNonce,
            c3Uuid: c3uuidkeeper,
            uuidNonce: uuidNoncePDA,
            ownerKey: owner,
            pause: pause

            
        })
        .rpc()

        console.log(`transaction tx: ${tx}`)
    })

   
})
