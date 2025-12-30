// Generated macro for ZeroingMethod (enum)
macro_rules! Depcrate_predicate_formsZeroingMethod {
() => {
// Module: crate::predicate_forms
// Provides: {"ZeroingMethod"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (untagged)] pub enum ZeroingMethod { # [doc = " Drop the specified argument and replace it with a zeroinitializer"] Drop { drop : WildString } , # [doc = " Apply zero selection to the specified variable when zeroing"] Select { select : WildString } , }
};
}
