// Generated macro for impl_744 (impl)
macro_rules! Depcrate_itemsimpl_744 {
() => {
// Module: crate::items
// Provides: {"impl_744"}
// Dependencies: {}
impl Rewrite for ast :: FnRetTy { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match * self { ast :: FnRetTy :: Default (_) => Ok (String :: new ()) , ast :: FnRetTy :: Ty (ref ty) => { let arrow_width = "-> " . len () ; if context . config . style_edition () <= StyleEdition :: Edition2021 || context . config . indent_style () == IndentStyle :: Visual { let inner_width = shape . width . checked_sub (arrow_width) . max_width_error (shape . width , self . span ()) ? ; return ty . rewrite_result (context , Shape :: legacy (inner_width , shape . indent + arrow_width) ,) . map (| r | format ! ("-> {}" , r)) ; } let shape = shape . offset_left (arrow_width , self . span ()) ? ; ty . rewrite_result (context , shape) . map (| s | format ! ("-> {}" , s)) } } } }
};
}
