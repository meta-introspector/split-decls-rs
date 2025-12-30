// Generated macro for block_can_be_flattened (function)
macro_rules! Depcrate_matchesblock_can_be_flattened {
() => {
// Module: crate::matches
// Provides: {"block_can_be_flattened"}
// Dependencies: {}
fn block_can_be_flattened < 'a > (context : & RewriteContext < '_ > , expr : & 'a ast :: Expr ,) -> Option < & 'a ast :: Block > { match expr . kind { ast :: ExprKind :: Block (ref block , label) if label . is_none () && ! is_unsafe_block (block) && ! context . inside_macro () && is_simple_block (context , block , Some (& expr . attrs)) && ! stmt_is_expr_mac (& block . stmts [0]) => { Some (& * block) } _ => None , } }
};
}
