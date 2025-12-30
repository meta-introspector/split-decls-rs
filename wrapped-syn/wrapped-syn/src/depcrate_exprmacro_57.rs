// Generated macro for macro_57 (macro)
macro_rules! Depcrate_exprmacro_57 {
() => {
// Module: crate::expr
// Provides: {"macro_57"}
// Dependencies: {}
# [cfg (feature = "full")] ast_enum ! { # [doc = " Limit types of a range (inclusive or exclusive)"] # [cfg_attr (feature = "clone-impls" , derive (Copy))] pub enum RangeLimits { # [doc = " Inclusive at the beginning, exclusive at the end"] HalfOpen (tokens :: Dot2) , # [doc = " Inclusive at the beginning and end"] Closed (tokens :: Dot3) , } }
};
}
