import * as anchor from "@coral-xyz/anchor";

export function setAnchor():[anchor.AnchorProvider, anchor.Wallet, string]{

    console.log(" anchor setup...")
    const provider = anchor.AnchorProvider.env();
    const rpc = provider.connection.rpcEndpoint;
    anchor.setProvider(provider)
    const wallet = provider.wallet as anchor.Wallet
    return [ provider, wallet, rpc]

}