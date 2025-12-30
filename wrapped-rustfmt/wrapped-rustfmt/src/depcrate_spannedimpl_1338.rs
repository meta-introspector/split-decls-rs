// Generated macro for impl_1338 (impl)
macro_rules! Depcrate_spannedimpl_1338 {
() => {
// Module: crate::spanned
// Provides: {"impl_1338"}
// Dependencies: {}
impl Spanned for ast :: Stmt { fn span (& self) -> Span { match self . kind { ast :: StmtKind :: Let (ref local) => mk_sp (local . span () . lo () , self . span . hi ()) , ast :: StmtKind :: Item (ref item) => mk_sp (item . span () . lo () , self . span . hi ()) , ast :: StmtKind :: Expr (ref expr) | ast :: StmtKind :: Semi (ref expr) => { mk_sp (expr . span () . lo () , self . span . hi ()) } ast :: StmtKind :: MacCall (ref mac_stmt) => { if mac_stmt . attrs . is_empty () { self . span } else { mk_sp (mac_stmt . attrs [0] . span . lo () , self . span . hi ()) } } ast :: StmtKind :: Empty => self . span , } } }
};
}
