// Generated macro for grouped (macro)
macro_rules! Depcrate_helpergrouped {
() => {
// Module: crate::helper
// Provides: {"grouped"}
// Dependencies: {}
# [doc = " Same as the `parens` macro, but for none-delimited sequences (groups)."] # [macro_export] macro_rules ! grouped { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *)) => { $ crate :: tokens :: Group :: parse ($ i , | i | $ submac ! (i , $ ($ args) *)) } ; ($ i : expr , $ f : expr) => { grouped ! ($ i , call ! ($ f)) ; } ; }
};
}
