// Generated macro for withdraw (function)
macro_rules! Depcrate_instructionwithdraw {
() => {
// Module: crate::instruction
// Provides: {"withdraw"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn withdraw (vote_pubkey : & Pubkey , authorized_withdrawer_pubkey : & Pubkey , lamports : u64 , to_pubkey : & Pubkey ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new (* to_pubkey , false) , AccountMeta :: new_readonly (* authorized_withdrawer_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: Withdraw (lamports) , account_metas) }
};
}
