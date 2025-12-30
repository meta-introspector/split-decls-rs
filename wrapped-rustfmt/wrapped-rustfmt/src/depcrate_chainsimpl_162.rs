// Generated macro for impl_162 (impl)
macro_rules! Depcrate_chainsimpl_162 {
() => {
// Module: crate::chains
// Provides: {"impl_162"}
// Dependencies: {}
impl ChainItem { fn new (context : & RewriteContext < '_ > , expr : & SubExpr , tries : usize) -> ChainItem { let (kind , span) = ChainItemKind :: from_ast (context , & expr . expr , expr . is_method_call_receiver) ; ChainItem { kind , tries , span } } fn comment (span : Span , comment : String , pos : CommentPosition) -> ChainItem { ChainItem { kind : ChainItemKind :: Comment (comment , pos) , tries : 0 , span , } } fn is_comment (& self) -> bool { matches ! (self . kind , ChainItemKind :: Comment (..)) } fn rewrite_method_call (method_name : symbol :: Ident , types : & [ast :: GenericArg] , args : & [ptr :: P < ast :: Expr >] , span : Span , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { let type_str = if types . is_empty () { String :: new () } else { let type_list = types . iter () . map (| ty | ty . rewrite_result (context , shape)) . collect :: < Result < Vec < _ > , RewriteError > > () ? ; format ! ("::<{}>" , type_list . join (", ")) } ; let callee_str = format ! (".{}{}" , rewrite_ident (context , method_name) , type_str) ; rewrite_call (context , & callee_str , & args , span , shape) } }
};
}
