use anchor_lang::solana_program::{keccak::hash, pubkey::Pubkey};
pub fn gen_uuid(sender:Pubkey,program_id:Pubkey,nonce: u64,dapp_id:u128,to:String, to_chain_id:String, data:[u8;32])-> [u8;32]{
    hash(
        &[
            &sender.to_bytes()[..],
            &program_id.to_bytes()[..],
            &nonce.to_be_bytes()[..],
            &dapp_id.to_be_bytes()[..],
            &to.as_bytes()[..],
            &to_chain_id.as_bytes()[..],
            &data[..]
        ]
        .concat()
    )
    .to_bytes()

}

