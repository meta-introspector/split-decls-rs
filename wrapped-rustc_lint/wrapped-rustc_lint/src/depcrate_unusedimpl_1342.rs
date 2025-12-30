// Generated macro for impl_1342 (impl)
macro_rules! Depcrate_unusedimpl_1342 {
() => {
// Module: crate::unused
// Provides: {"impl_1342"}
// Dependencies: {}
impl From < UnusedDelimsCtx > for & 'static str { fn from (ctx : UnusedDelimsCtx) -> & 'static str { match ctx { UnusedDelimsCtx :: FunctionArg => "function argument" , UnusedDelimsCtx :: MethodArg => "method argument" , UnusedDelimsCtx :: AssignedValue | UnusedDelimsCtx :: AssignedValueLetElse => { "assigned value" } UnusedDelimsCtx :: IfCond => "`if` condition" , UnusedDelimsCtx :: WhileCond => "`while` condition" , UnusedDelimsCtx :: ForIterExpr => "`for` iterator expression" , UnusedDelimsCtx :: MatchScrutineeExpr => "`match` scrutinee expression" , UnusedDelimsCtx :: ReturnValue => "`return` value" , UnusedDelimsCtx :: BlockRetValue => "block return value" , UnusedDelimsCtx :: BreakValue => "`break` value" , UnusedDelimsCtx :: LetScrutineeExpr => "`let` scrutinee expression" , UnusedDelimsCtx :: ArrayLenExpr | UnusedDelimsCtx :: AnonConst => "const expression" , UnusedDelimsCtx :: MatchArmExpr => "match arm expression" , UnusedDelimsCtx :: IndexExpr => "index expression" , UnusedDelimsCtx :: ClosureBody => "closure body" , } } }
};
}
