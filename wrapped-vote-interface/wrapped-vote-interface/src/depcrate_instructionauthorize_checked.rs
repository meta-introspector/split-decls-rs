// Generated macro for authorize_checked (function)
macro_rules! Depcrate_instructionauthorize_checked {
() => {
// Module: crate::instruction
// Provides: {"authorize_checked"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn authorize_checked (vote_pubkey : & Pubkey , authorized_pubkey : & Pubkey , new_authorized_pubkey : & Pubkey , vote_authorize : VoteAuthorize ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (* authorized_pubkey , true) , AccountMeta :: new_readonly (* new_authorized_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: AuthorizeChecked (vote_authorize) , account_metas ,) }
};
}
