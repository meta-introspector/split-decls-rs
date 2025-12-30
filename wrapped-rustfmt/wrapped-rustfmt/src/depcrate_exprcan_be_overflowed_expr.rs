// Generated macro for can_be_overflowed_expr (function)
macro_rules! Depcrate_exprcan_be_overflowed_expr {
() => {
// Module: crate::expr
// Provides: {"can_be_overflowed_expr"}
// Dependencies: {}
pub (crate) fn can_be_overflowed_expr (context : & RewriteContext < '_ > , expr : & ast :: Expr , args_len : usize ,) -> bool { match expr . kind { _ if ! expr . attrs . is_empty () => false , ast :: ExprKind :: Match (..) => { (context . use_block_indent () && args_len == 1) || (context . config . indent_style () == IndentStyle :: Visual && args_len > 1) || context . config . overflow_delimited_expr () } ast :: ExprKind :: If (..) | ast :: ExprKind :: ForLoop { .. } | ast :: ExprKind :: Loop (..) | ast :: ExprKind :: While (..) => { context . config . combine_control_expr () && context . use_block_indent () && args_len == 1 } ast :: ExprKind :: Gen (..) | ast :: ExprKind :: Block (..) | ast :: ExprKind :: Closure (..) => true , ast :: ExprKind :: Array (..) | ast :: ExprKind :: Struct (..) => { context . config . overflow_delimited_expr () || (context . use_block_indent () && args_len == 1) } ast :: ExprKind :: MacCall (ref mac) => { match (mac . args . delim , context . config . overflow_delimited_expr ()) { (Delimiter :: Bracket , true) | (Delimiter :: Brace , true) => true , _ => context . use_block_indent () && args_len == 1 , } } ast :: ExprKind :: Call (..) | ast :: ExprKind :: MethodCall (..) | ast :: ExprKind :: Tup (..) => { context . use_block_indent () && args_len == 1 } ast :: ExprKind :: AddrOf (_ , _ , ref expr) | ast :: ExprKind :: Try (ref expr) | ast :: ExprKind :: Unary (_ , ref expr) | ast :: ExprKind :: Cast (ref expr , _) => can_be_overflowed_expr (context , expr , args_len) , _ => false , } }
};
}
