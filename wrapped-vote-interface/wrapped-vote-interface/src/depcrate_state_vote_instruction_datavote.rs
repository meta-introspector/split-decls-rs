// Generated macro for Vote (struct)
macro_rules! Depcrate_state_vote_instruction_dataVote {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"Vote"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , frozen_abi (digest = "GvUzgtcxhKVVxPAjSntXGPqjLZK5ovgZzCiUP1tDpB9q") , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Default , Debug , PartialEq , Eq , Clone)] pub struct Vote { # [doc = " A stack of votes starting with the oldest vote"] pub slots : Vec < Slot > , # [doc = " signature of the bank's state at the last slot"] pub hash : Hash , # [doc = " processing timestamp of last slot"] pub timestamp : Option < UnixTimestamp > , }
};
}
