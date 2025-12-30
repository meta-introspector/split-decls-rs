// Generated macro for VoteAuthorizeWithSeedArgs (struct)
macro_rules! Depcrate_state_vote_instruction_dataVoteAuthorizeWithSeedArgs {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"VoteAuthorizeWithSeedArgs"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , PartialEq , Eq , Clone)] pub struct VoteAuthorizeWithSeedArgs { pub authorization_type : VoteAuthorize , pub current_authority_derived_key_owner : Pubkey , pub current_authority_derived_key_seed : String , pub new_authority : Pubkey , }
};
}
