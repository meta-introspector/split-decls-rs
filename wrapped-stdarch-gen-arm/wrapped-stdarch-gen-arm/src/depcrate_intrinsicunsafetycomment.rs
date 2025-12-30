// Generated macro for UnsafetyComment (enum)
macro_rules! Depcrate_intrinsicUnsafetyComment {
() => {
// Module: crate::intrinsic
// Provides: {"UnsafetyComment"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub enum UnsafetyComment { Custom (String) , Uninitialized , PointerOffset (GovernedBy) , PointerOffsetVnum (GovernedBy) , Dereference (GovernedBy) , UnpredictableOnFault , NonTemporal , Neon , NoProvenance (String) , }
};
}
