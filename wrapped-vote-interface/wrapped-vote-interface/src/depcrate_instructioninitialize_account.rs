// Generated macro for initialize_account (function)
macro_rules! Depcrate_instructioninitialize_account {
() => {
// Module: crate::instruction
// Provides: {"initialize_account"}
// Dependencies: {}
# [cfg (feature = "bincode")] fn initialize_account (vote_pubkey : & Pubkey , vote_init : & VoteInit) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (sysvar :: rent :: id () , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (vote_init . node_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: InitializeAccount (* vote_init) , account_metas ,) }
};
}
