// Generated macro for create_nonce_account_with_seed (function)
macro_rules! Depcrate_instructioncreate_nonce_account_with_seed {
() => {
// Module: crate::instruction
// Provides: {"create_nonce_account_with_seed"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn create_nonce_account_with_seed (from_address : & Address , nonce_address : & Address , base : & Address , seed : & str , authority : & Address , lamports : u64 ,) -> Vec < Instruction > { vec ! [create_account_with_seed (from_address , nonce_address , base , seed , lamports , NONCE_STATE_SIZE as u64 , & ID ,) , Instruction :: new_with_bincode (ID , & SystemInstruction :: InitializeNonceAccount (* authority) , vec ! [AccountMeta :: new (* nonce_address , false) , # [allow (deprecated)] AccountMeta :: new_readonly (RECENT_BLOCKHASHES_ID , false) , AccountMeta :: new_readonly (RENT_ID , false) ,] ,) ,] }
};
}
