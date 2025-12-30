// Generated macro for Binder (struct)
macro_rules! Depcrate_tyBinder {
() => {
// Module: crate::ty
// Provides: {"Binder"}
// Dependencies: {}
# [doc = " A binder represents a possibly generic type and its bound vars."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Binder < T > { pub value : T , pub bound_vars : Vec < BoundVariableKind > , }
};
}
