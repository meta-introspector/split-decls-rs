// Generated macro for macro_357 (macro)
macro_rules! Depcrate_exprmacro_357 {
() => {
// Module: crate::expr
// Provides: {"macro_357"}
// Dependencies: {}
# [cfg (feature = "full")] ast_enum ! { # [doc = " Limit types of a range, inclusive or exclusive."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub enum RangeLimits { # [doc = " Inclusive at the beginning, exclusive at the end."] HalfOpen (Token ! [..]) , # [doc = " Inclusive at the beginning and end."] Closed (Token ! [..=]) , } }
};
}
