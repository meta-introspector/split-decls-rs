// Generated macro for initialize_account_v2 (function)
macro_rules! Depcrate_instructioninitialize_account_v2 {
() => {
// Module: crate::instruction
// Provides: {"initialize_account_v2"}
// Dependencies: {}
# [cfg (feature = "bincode")] fn initialize_account_v2 (vote_pubkey : & Pubkey , vote_init : & VoteInitV2) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (vote_init . node_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: InitializeAccountV2 (* vote_init) , account_metas ,) }
};
}
