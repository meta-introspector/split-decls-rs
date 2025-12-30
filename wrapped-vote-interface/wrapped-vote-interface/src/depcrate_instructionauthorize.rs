// Generated macro for authorize (function)
macro_rules! Depcrate_instructionauthorize {
() => {
// Module: crate::instruction
// Provides: {"authorize"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn authorize (vote_pubkey : & Pubkey , authorized_pubkey : & Pubkey , new_authorized_pubkey : & Pubkey , vote_authorize : VoteAuthorize ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (* authorized_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: Authorize (* new_authorized_pubkey , vote_authorize) , account_metas ,) }
};
}
