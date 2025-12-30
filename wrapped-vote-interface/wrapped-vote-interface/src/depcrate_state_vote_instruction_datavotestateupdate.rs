// Generated macro for VoteStateUpdate (struct)
macro_rules! Depcrate_state_vote_instruction_dataVoteStateUpdate {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"VoteStateUpdate"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , frozen_abi (digest = "CxyuwbaEdzP7jDCZyxjgQvLGXadBUZF3LoUvbSpQ6tYN") , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Default , Debug , PartialEq , Eq , Clone)] pub struct VoteStateUpdate { # [doc = " The proposed tower"] pub lockouts : VecDeque < Lockout > , # [doc = " The proposed root"] pub root : Option < Slot > , # [doc = " signature of the bank's state at the last slot"] pub hash : Hash , # [doc = " processing timestamp of last slot"] pub timestamp : Option < UnixTimestamp > , }
};
}
