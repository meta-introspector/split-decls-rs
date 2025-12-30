// Generated macro for macro_809 (macro)
macro_rules! Depcrate_pathmacro_809 {
() => {
// Module: crate::path
// Provides: {"macro_809"}
// Dependencies: {}
ast_struct ! { # [doc = " A path at which a named item is exported (e.g. `std::collections::HashMap`)."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Path { pub leading_colon : Option < Token ! [::] >, pub segments : Punctuated < PathSegment , Token ! [::] >, } }
};
}
