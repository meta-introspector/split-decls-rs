// Generated macro for TowerSync (struct)
macro_rules! Depcrate_state_vote_instruction_dataTowerSync {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"TowerSync"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , frozen_abi (digest = "6UDiQMH4wbNwkMHosPMtekMYu2Qa6CHPZ2ymK4mc6FGu") , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Default , Debug , PartialEq , Eq , Clone)] pub struct TowerSync { # [doc = " The proposed tower"] pub lockouts : VecDeque < Lockout > , # [doc = " The proposed root"] pub root : Option < Slot > , # [doc = " signature of the bank's state at the last slot"] pub hash : Hash , # [doc = " processing timestamp of last slot"] pub timestamp : Option < UnixTimestamp > , # [doc = " the unique identifier for the chain up to and"] # [doc = " including this block. Does not require replaying"] # [doc = " in order to compute."] pub block_id : Hash , }
};
}
