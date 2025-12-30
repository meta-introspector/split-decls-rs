// Generated macro for RangeItem (trait)
macro_rules! Depcrate_astRangeItem {
() => {
// Module: crate::ast
// Provides: {"RangeItem"}
// Dependencies: {}
# [doc = " Trait to describe operations common to both `RangeExpr` and `RangePat`."] pub trait RangeItem { type Bound ; fn start (& self) -> Option < Self :: Bound > ; fn end (& self) -> Option < Self :: Bound > ; fn op_kind (& self) -> Option < RangeOp > ; fn op_token (& self) -> Option < SyntaxToken > ; }
};
}
