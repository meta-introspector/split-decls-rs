// Generated macro for CircBuf (struct)
macro_rules! Depcrate_stateCircBuf {
() => {
// Module: crate::state
// Provides: {"CircBuf"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [derive (Debug , PartialEq , Eq , Clone)] # [cfg_attr (feature = "dev-context-only-utils" , derive (Arbitrary))] pub struct CircBuf < I > { buf : [I ; MAX_ITEMS] , # [doc = " next pointer"] idx : usize , is_empty : bool , }
};
}
