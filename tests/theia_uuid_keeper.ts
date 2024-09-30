import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TheiaUuidKeeper } from "../target/types/theia_uuid_keeper";
import { Context } from "mocha";

describe("theiaUUIDKeeper", () => {

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider)

    const systemProgram = anchor.web3.SystemProgram
   
    const program =  anchor.workspace.TheiaUuidKeeper as Program<TheiaUuidKeeper>

    const CURRENT_NONCE_SEED = Buffer.from("current_nonce");
    const [currentNonceAccount,_bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [CURRENT_NONCE_SEED],
      program.programId
    );



    it("Is initialized", async()=>{

        const tx = await program.methods.initialize()
        .accounts({  currentNonce:currentNonceAccount })
        .rpc();


    console.log("Your transaction signature", tx);
    })

    it("generate UUID", async()=>{

        
        const  currentNoncePda = await program.account.currentNonce.fetch(currentNonceAccount);
         const currentNonce = currentNoncePda.nonce;
    
       // 3. Calculate the PDA for the uuid_nonce account
        const [uuidNoncePDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("uuid_nonce"),
            Buffer.from((currentNonce.add(new anchor.BN(1)).toArrayLike(Buffer, 'be', 8)))
          ],
          program.programId
        );


        let tx = await program.methods.genUuidEvm({
            token: "exampleToken",
            from: "0x1234567890123456789012345678901234567890",
            amount: new anchor.BN(1000),
            receiver: "0x0987654321098765432109876543210987654321",
            toChainId: new anchor.BN(1)
        })
        .accounts({
            currentNonce: currentNonceAccount,
            uuidNonce: uuidNoncePDA,
        })
        .rpc();
        console.log("Your transaction signature", tx);

    })


})