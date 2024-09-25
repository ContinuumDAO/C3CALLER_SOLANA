import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Theia } from "../target/types/theia";

describe("theia", () => {

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider)

    const systemProgram = anchor.web3.SystemProgram
    const program = anchor.workspace.Theia as Program<Theia>



    it("Is initialized!", async()=>{

        console.log("hello world")
    })
})
