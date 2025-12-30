// Generated macro for is_block_closure_forced_inner (function)
macro_rules! Depcrate_closuresis_block_closure_forced_inner {
() => {
// Module: crate::closures
// Provides: {"is_block_closure_forced_inner"}
// Dependencies: {}
fn is_block_closure_forced_inner (expr : & ast :: Expr , style_edition : StyleEdition) -> bool { match expr . kind { ast :: ExprKind :: If (..) | ast :: ExprKind :: While (..) | ast :: ExprKind :: ForLoop { .. } => true , ast :: ExprKind :: Loop (..) if style_edition >= StyleEdition :: Edition2024 => true , ast :: ExprKind :: AddrOf (_ , _ , ref expr) | ast :: ExprKind :: Try (ref expr) | ast :: ExprKind :: Unary (_ , ref expr) | ast :: ExprKind :: Cast (ref expr , _) => is_block_closure_forced_inner (expr , style_edition) , _ => false , } }
};
}
