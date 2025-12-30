// Generated macro for VoteStateVersions (enum)
macro_rules! Depcrate_state_vote_state_versionsVoteStateVersions {
() => {
// Module: crate::state::vote_state_versions
// Provides: {"VoteStateVersions"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Debug , PartialEq , Eq , Clone)] pub enum VoteStateVersions { Uninitialized , V1_14_11 (Box < VoteState1_14_11 >) , V3 (Box < VoteStateV3 >) , V4 (Box < VoteStateV4 >) , }
};
}
