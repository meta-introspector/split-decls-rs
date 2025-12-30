// Generated macro for impl_1548 (impl)
macro_rules! Depcrate_typesimpl_1548 {
() => {
// Module: crate::types
// Provides: {"impl_1548"}
// Dependencies: {}
impl Rewrite for ast :: GenericBound { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match * self { ast :: GenericBound :: Trait (ref poly_trait_ref) => { let snippet = context . snippet (self . span ()) ; let has_paren = snippet . starts_with ('(') && snippet . ends_with (')') ; poly_trait_ref . rewrite_result (context , shape) . map (| s | if has_paren { format ! ("({})" , s) } else { s }) } ast :: GenericBound :: Use (ref args , span) => { overflow :: rewrite_with_angle_brackets (context , "use" , args . iter () , shape , span) } ast :: GenericBound :: Outlives (ref lifetime) => lifetime . rewrite_result (context , shape) , } } }
};
}
