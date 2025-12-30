// Generated macro for LandedVote (struct)
macro_rules! Depcrate_stateLandedVote {
() => {
// Module: crate::state
// Provides: {"LandedVote"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Default , Debug , PartialEq , Eq , Copy , Clone)] # [cfg_attr (feature = "dev-context-only-utils" , derive (Arbitrary))] pub struct LandedVote { pub latency : u8 , pub lockout : Lockout , }
};
}
