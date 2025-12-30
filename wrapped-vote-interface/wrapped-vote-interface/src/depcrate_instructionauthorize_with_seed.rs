// Generated macro for authorize_with_seed (function)
macro_rules! Depcrate_instructionauthorize_with_seed {
() => {
// Module: crate::instruction
// Provides: {"authorize_with_seed"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn authorize_with_seed (vote_pubkey : & Pubkey , current_authority_base_key : & Pubkey , current_authority_derived_key_owner : & Pubkey , current_authority_derived_key_seed : & str , new_authority : & Pubkey , authorization_type : VoteAuthorize ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (* current_authority_base_key , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: AuthorizeWithSeed (VoteAuthorizeWithSeedArgs { authorization_type , current_authority_derived_key_owner : * current_authority_derived_key_owner , current_authority_derived_key_seed : current_authority_derived_key_seed . to_string () , new_authority : * new_authority , }) , account_metas ,) }
};
}
