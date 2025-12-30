// Generated macro for impl_26 (impl)
macro_rules! Depcrate_astimpl_26 {
() => {
// Module: crate::ast
// Provides: {"impl_26"}
// Dependencies: {}
impl Assign { # [doc = " Creates a new variable assignment, where `value` is assigned"] # [doc = " to `var`."] # [doc = ""] # [doc = " The assignment can be a compound assignment when `op` is different"] # [doc = " from [=](BinOp::Eq) (e.g. `x+=1`)."] pub (crate) fn new (var : SubscriptedVar , value : impl Into < Value > , op : BinOp) -> Self { Self { var , value : value . into () , op , } } }
};
}
