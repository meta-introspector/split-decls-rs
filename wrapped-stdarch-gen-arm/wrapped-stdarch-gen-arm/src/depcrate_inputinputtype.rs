// Generated macro for InputType (enum)
macro_rules! Depcrate_inputInputType {
() => {
// Module: crate::input
// Provides: {"InputType"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (untagged)] pub enum InputType { # [doc = " PredicateForm variant argument"] # [serde (skip)] PredicateForm (PredicateForm) , # [doc = " Operand from which to generate an N variant"] # [serde (skip)] NVariantOp (Option < WildString >) , # [doc = " TypeKind variant argument"] Type (TypeKind) , }
};
}
