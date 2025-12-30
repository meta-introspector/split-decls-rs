// Generated macro for ast_enum_of_structs_impl (macro)
macro_rules! Depcrate_macrosast_enum_of_structs_impl {
() => {
// Module: crate::macros
// Provides: {"ast_enum_of_structs_impl"}
// Dependencies: {}
macro_rules ! ast_enum_of_structs_impl { ($ name : ident { $ ($ (# [cfg $ cfg_attr : tt]) * $ (# [doc $ ($ doc_attr : tt) *]) * $ variant : ident $ (($ member : ident)) *,) * }) => { $ ($ (ast_enum_from_struct ! ($ name ::$ variant , $ member) ;) *) * } ; }
};
}
