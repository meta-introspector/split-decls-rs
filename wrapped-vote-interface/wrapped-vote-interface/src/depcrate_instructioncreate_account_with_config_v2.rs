// Generated macro for create_account_with_config_v2 (function)
macro_rules! Depcrate_instructioncreate_account_with_config_v2 {
() => {
// Module: crate::instruction
// Provides: {"create_account_with_config_v2"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn create_account_with_config_v2 (from_pubkey : & Pubkey , vote_pubkey : & Pubkey , vote_init : & VoteInitV2 , lamports : u64 , config : CreateVoteAccountConfig ,) -> Vec < Instruction > { let create_ix = if let Some ((base , seed)) = config . with_seed { solana_system_interface :: instruction :: create_account_with_seed (from_pubkey , vote_pubkey , base , seed , lamports , config . space , & id () ,) } else { solana_system_interface :: instruction :: create_account (from_pubkey , vote_pubkey , lamports , config . space , & id () ,) } ; let init_ix = initialize_account_v2 (vote_pubkey , vote_init) ; vec ! [create_ix , init_ix] }
};
}
