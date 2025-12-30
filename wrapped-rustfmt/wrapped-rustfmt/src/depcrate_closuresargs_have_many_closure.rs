// Generated macro for args_have_many_closure (function)
macro_rules! Depcrate_closuresargs_have_many_closure {
() => {
// Module: crate::closures
// Provides: {"args_have_many_closure"}
// Dependencies: {}
# [doc = " Returns `true` if the given vector of arguments has more than one `ast::ExprKind::Closure`."] pub (crate) fn args_have_many_closure (args : & [OverflowableItem < '_ >]) -> bool { args . iter () . filter_map (OverflowableItem :: to_expr) . filter (| expr | matches ! (expr . kind , ast :: ExprKind :: Closure (..))) . count () > 1 }
};
}
