// Generated macro for brackets (macro)
macro_rules! Depcrate_helperbrackets {
() => {
// Module: crate::helper
// Provides: {"brackets"}
// Dependencies: {}
# [doc = " Same as the `parens` macro, but for brackets."] # [macro_export] macro_rules ! brackets { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *)) => { $ crate :: tokens :: Bracket :: parse ($ i , | i | $ submac ! (i , $ ($ args) *)) } ; ($ i : expr , $ f : expr) => { brackets ! ($ i , call ! ($ f)) ; } ; }
};
}
