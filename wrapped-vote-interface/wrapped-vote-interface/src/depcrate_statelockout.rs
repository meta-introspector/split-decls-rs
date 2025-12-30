// Generated macro for Lockout (struct)
macro_rules! Depcrate_stateLockout {
() => {
// Module: crate::state
// Provides: {"Lockout"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Default , Debug , PartialEq , Eq , Copy , Clone)] # [cfg_attr (feature = "dev-context-only-utils" , derive (Arbitrary))] pub struct Lockout { slot : Slot , confirmation_count : u32 , }
};
}
