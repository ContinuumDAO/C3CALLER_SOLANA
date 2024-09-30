use anchor_lang::solana_program::{keccak::hash, pubkey::Pubkey};
use crate::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct C3SolMessage {
    pub uuid:[u8;32],
    pub to:String,
    pub from_chain_id:String,
    pub source_tx:String,
    pub fallback_to:String,
    pub data:ExecuteParams,
}
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct  C3Context{
    pub swap_id:[u8;32],
    pub from_chain_id:String,
    pub source_tx:String
}


#[account]
pub struct  Pause{
    pub is_paused:bool
}

#[account]
pub  struct OwnerKey{
    pub onwer_key:Pubkey
}
#[account]

pub struct C3UUIDKeeper{
     pub current_nonce: u64,
     pub uuid_2_nonce: Vec<[u8; 32]>,
}

impl C3UUIDKeeper {

   
    pub fn gen_uuid(&mut self,sender:Pubkey,program_id:Pubkey,dapp_id:u128,to:String, to_chain_id:String, data:[u8;32])-> [u8;32]{
        let _nonce = self.current_nonce +1;
        hash(
            &[
                &sender.to_bytes()[..],
                &program_id.to_bytes()[..],
                &_nonce.to_be_bytes()[..],
                &dapp_id.to_be_bytes()[..],
                &to.as_bytes()[..],
                &to_chain_id.as_bytes()[..],
                &data[..]
            ]
            .concat()
        )
        .to_bytes()
    
}
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ExecuteParams {
    // target program to execute against
    pub program_id: Pubkey,
    // accounts required for the transaction
    pub accounts: Vec<TransactionAccount>,
    pub data: Vec<u8>,
   
   
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<&TransactionAccount> for AccountMeta {
    fn from(account: &TransactionAccount) -> AccountMeta {
        match account.is_writable {
            false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
            true => AccountMeta::new(account.pubkey, account.is_signer),
        }
    }
}