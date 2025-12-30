// Generated macro for impl_1044 (impl)
macro_rules! Depcrate_pairsimpl_1044 {
() => {
// Module: crate::pairs
// Provides: {"impl_1044"}
// Dependencies: {}
impl < 'a , 'b > PairList < 'a , 'b , ast :: Expr > { fn let_chain_count (& self) -> usize { self . list . iter () . filter (| (expr , _) | matches ! (expr . kind , ast :: ExprKind :: Let (..))) . count () } fn can_rewrite_let_chain_single_line (& self) -> bool { if self . list . len () != 2 { return false ; } let fist_item_is_ident_or_bool_lit = is_ident_or_bool_lit (self . list [0] . 0) ; let second_item_is_let_chain = matches ! (self . list [1] . 0 . kind , ast :: ExprKind :: Let (..)) ; fist_item_is_ident_or_bool_lit && second_item_is_let_chain } }
};
}
