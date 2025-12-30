// Generated macro for BlockTimestamp (struct)
macro_rules! Depcrate_stateBlockTimestamp {
() => {
// Module: crate::state
// Provides: {"BlockTimestamp"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , Default , PartialEq , Eq , Clone)] # [cfg_attr (feature = "dev-context-only-utils" , derive (Arbitrary))] pub struct BlockTimestamp { pub slot : Slot , pub timestamp : UnixTimestamp , }
};
}
