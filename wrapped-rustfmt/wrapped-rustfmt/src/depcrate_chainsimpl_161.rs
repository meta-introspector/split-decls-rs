// Generated macro for impl_161 (impl)
macro_rules! Depcrate_chainsimpl_161 {
() => {
// Module: crate::chains
// Provides: {"impl_161"}
// Dependencies: {}
impl Rewrite for ChainItem { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { let shape = shape . sub_width (self . tries , self . span) ? ; let rewrite = match self . kind { ChainItemKind :: Parent { ref expr , parens : true , } => crate :: expr :: rewrite_paren (context , & expr , shape , expr . span) ? , ChainItemKind :: Parent { ref expr , parens : false , } => expr . rewrite_result (context , shape) ? , ChainItemKind :: MethodCall (ref segment , ref types , ref exprs) => { Self :: rewrite_method_call (segment . ident , types , exprs , self . span , context , shape) ? } ChainItemKind :: StructField (ident) => format ! (".{}" , rewrite_ident (context , ident)) , ChainItemKind :: TupleField (ident , nested) => format ! ("{}.{}" , if nested && context . config . style_edition () <= StyleEdition :: Edition2021 { " " } else { "" } , rewrite_ident (context , ident)) , ChainItemKind :: Await => ".await" . to_owned () , ChainItemKind :: Use => ".use" . to_owned () , ChainItemKind :: Yield => ".yield" . to_owned () , ChainItemKind :: Comment (ref comment , _) => { rewrite_comment (comment , false , shape , context . config) ? } } ; Ok (format ! ("{rewrite}{}" , "?" . repeat (self . tries))) } }
};
}
