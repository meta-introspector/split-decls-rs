// Generated macro for macro_825 (macro)
macro_rules! Depcrate_pathmacro_825 {
() => {
// Module: crate::path
// Provides: {"macro_825"}
// Dependencies: {}
ast_struct ! { # [doc = " A path at which a named item is exported (e.g. `std::collections::HashMap`)."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Path { pub leading_colon : Option < Token ! [::] >, pub segments : Punctuated < PathSegment , Token ! [::] >, } }
};
}
