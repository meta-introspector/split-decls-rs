// Generated macro for impl_1343 (impl)
macro_rules! Depcrate_stmtimpl_1343 {
() => {
// Module: crate::stmt
// Provides: {"impl_1343"}
// Dependencies: {}
impl < 'a > Stmt < 'a > { pub (crate) fn as_ast_node (& self) -> & ast :: Stmt { self . inner } pub (crate) fn to_item (& self) -> Option < & ast :: Item > { match self . inner . kind { ast :: StmtKind :: Item (ref item) => Some (& * * item) , _ => None , } } pub (crate) fn from_simple_block (context : & RewriteContext < '_ > , block : & 'a ast :: Block , attrs : Option < & [ast :: Attribute] > ,) -> Option < Self > { if is_simple_block (context , block , attrs) { let inner = & block . stmts [0] ; let is_last = true ; Some (Stmt { inner , is_last }) } else { None } } pub (crate) fn from_ast_node (inner : & 'a ast :: Stmt , is_last : bool) -> Self { Stmt { inner , is_last } } pub (crate) fn from_ast_nodes < I > (iter : I) -> Vec < Self > where I : Iterator < Item = & 'a ast :: Stmt > , { let mut result = vec ! [] ; let mut iter = iter . peekable () ; while iter . peek () . is_some () { result . push (Stmt { inner : iter . next () . unwrap () , is_last : iter . peek () . is_none () , }) } result } pub (crate) fn is_empty (& self) -> bool { matches ! (self . inner . kind , ast :: StmtKind :: Empty) } fn is_last_expr (& self) -> bool { if ! self . is_last { return false ; } match self . as_ast_node () . kind { ast :: StmtKind :: Expr (ref expr) => match expr . kind { ast :: ExprKind :: Ret (..) | ast :: ExprKind :: Continue (..) | ast :: ExprKind :: Break (..) => { false } _ => true , } , _ => false , } } }
};
}
