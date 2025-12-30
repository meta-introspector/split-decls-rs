// Generated macro for fix_ident (function)
macro_rules! Depcrate_utilfix_ident {
() => {
// Module: crate::util
// Provides: {"fix_ident"}
// Dependencies: {}
# [doc = " Fix case of identifiers like `HTMLBRElement` or `texImage2D`"] fn fix_ident (identifier : & str) -> String { identifier . replace ("HTML" , "HTML_") . replace ("1D" , "_1d") . replace ("2D" , "_2d") . replace ("3D" , "_3d") }
};
}
