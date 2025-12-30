// Generated macro for VoteAuthorize (enum)
macro_rules! Depcrate_state_vote_instruction_dataVoteAuthorize {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"VoteAuthorize"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum VoteAuthorize { Voter , Withdrawer , VoterWithBLS (VoterWithBLSArgs) , }
};
}
