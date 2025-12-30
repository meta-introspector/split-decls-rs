// Generated macro for authorize_checked_with_seed (function)
macro_rules! Depcrate_instructionauthorize_checked_with_seed {
() => {
// Module: crate::instruction
// Provides: {"authorize_checked_with_seed"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn authorize_checked_with_seed (vote_pubkey : & Pubkey , current_authority_base_key : & Pubkey , current_authority_derived_key_owner : & Pubkey , current_authority_derived_key_seed : & str , new_authority : & Pubkey , authorization_type : VoteAuthorize ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (* current_authority_base_key , true) , AccountMeta :: new_readonly (* new_authority , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: AuthorizeCheckedWithSeed (VoteAuthorizeCheckedWithSeedArgs { authorization_type , current_authority_derived_key_owner : * current_authority_derived_key_owner , current_authority_derived_key_seed : current_authority_derived_key_seed . to_string () , }) , account_metas ,) }
};
}
