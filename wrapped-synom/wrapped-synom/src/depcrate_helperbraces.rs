// Generated macro for braces (macro)
macro_rules! Depcrate_helperbraces {
() => {
// Module: crate::helper
// Provides: {"braces"}
// Dependencies: {}
# [doc = " Same as the `parens` macro, but for braces."] # [macro_export] macro_rules ! braces { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *)) => { $ crate :: tokens :: Brace :: parse ($ i , | i | $ submac ! (i , $ ($ args) *)) } ; ($ i : expr , $ f : expr) => { braces ! ($ i , call ! ($ f)) ; } ; }
};
}
