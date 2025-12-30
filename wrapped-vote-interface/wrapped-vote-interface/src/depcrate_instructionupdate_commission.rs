// Generated macro for update_commission (function)
macro_rules! Depcrate_instructionupdate_commission {
() => {
// Module: crate::instruction
// Provides: {"update_commission"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn update_commission (vote_pubkey : & Pubkey , authorized_withdrawer_pubkey : & Pubkey , commission : u8 ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (* authorized_withdrawer_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: UpdateCommission (commission) , account_metas ,) }
};
}
