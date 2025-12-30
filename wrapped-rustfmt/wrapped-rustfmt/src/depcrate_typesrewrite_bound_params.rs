// Generated macro for rewrite_bound_params (function)
macro_rules! Depcrate_typesrewrite_bound_params {
() => {
// Module: crate::types
// Provides: {"rewrite_bound_params"}
// Dependencies: {}
# [doc = " Returns `None` if there is no `GenericParam` in the list"] pub (crate) fn rewrite_bound_params (context : & RewriteContext < '_ > , shape : Shape , generic_params : & [ast :: GenericParam] ,) -> Option < String > { let result = generic_params . iter () . map (| param | param . rewrite (context , shape)) . collect :: < Option < Vec < _ > > > () ? . join (", ") ; if result . is_empty () { None } else { Some (result) } }
};
}
