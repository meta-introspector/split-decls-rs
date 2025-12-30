// Generated macro for VoteAuthorizeCheckedWithSeedArgs (struct)
macro_rules! Depcrate_state_vote_instruction_dataVoteAuthorizeCheckedWithSeedArgs {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"VoteAuthorizeCheckedWithSeedArgs"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , PartialEq , Eq , Clone)] pub struct VoteAuthorizeCheckedWithSeedArgs { pub authorization_type : VoteAuthorize , pub current_authority_derived_key_owner : Pubkey , pub current_authority_derived_key_seed : String , }
};
}
