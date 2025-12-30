// Generated macro for impl_1349 (impl)
macro_rules! Depcrate_unusedimpl_1349 {
() => {
// Module: crate::unused
// Provides: {"impl_1349"}
// Dependencies: {}
impl UnusedParens { fn check_unused_parens_pat (& self , cx : & EarlyContext < '_ > , value : & ast :: Pat , avoid_or : bool , avoid_mut : bool , keep_space : (bool , bool) ,) { use ast :: { BindingMode , PatKind } ; if let PatKind :: Paren (inner) = & value . kind { match inner . kind { PatKind :: Range (..) => return , PatKind :: Or (..) if avoid_or => return , PatKind :: Ident (BindingMode :: MUT , ..) if avoid_mut => { return ; } _ => { } } let spans = if ! value . span . from_expansion () { inner . span . find_ancestor_inside (value . span) . map (| inner | (value . span . with_hi (inner . lo ()) , value . span . with_lo (inner . hi ()))) } else { None } ; self . emit_unused_delims (cx , value . span , spans , "pattern" , keep_space , false) ; } } fn cast_followed_by_lt (& self , expr : & ast :: Expr) -> Option < ast :: NodeId > { if let ExprKind :: Binary (op , lhs , _rhs) = & expr . kind && (op . node == ast :: BinOpKind :: Lt || op . node == ast :: BinOpKind :: Shl) { let mut cur = lhs ; while let ExprKind :: Binary (_ , _ , rhs) = & cur . kind { cur = rhs ; } if let ExprKind :: Cast (_ , ty) = & cur . kind && let ast :: TyKind :: Paren (_) = & ty . kind { return Some (ty . id) ; } } None } }
};
}
