import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Theia } from "../target/types/theia";
import { TheiaUuidKeeper } from "../target/types/theia_uuid_keeper";
import { C3callerSolana } from "../target/types/c_3caller_solana";

describe("theia", () => {

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider)

    const systemProgram = anchor.web3.SystemProgram
    const theia = anchor.workspace.Theia as Program<Theia>
    const theiaUuidKeeper =  anchor.workspace.TheiaUuidKeeper as Program<TheiaUuidKeeper>
    



    it("Is initialized!", async()=>{

        console.log("hello world")
    })
})
