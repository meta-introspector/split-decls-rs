// Generated macro for ast_enum_of_structs (macro)
macro_rules! Depcrate_macrosast_enum_of_structs {
() => {
// Module: crate::macros
// Provides: {"ast_enum_of_structs"}
// Dependencies: {}
macro_rules ! ast_enum_of_structs { ($ (# [$ enum_attr : meta]) * pub enum $ name : ident { $ ($ (# [$ variant_attr : meta]) * pub $ variant : ident ($ member : ident $ ($ rest : tt) *) ,) * } $ ($ remaining : tt) *) => (ast_enum ! { $ (# [$ enum_attr]) * pub enum $ name { $ ($ (# [$ variant_attr]) * $ variant ($ member) ,) * } } $ (maybe_ast_struct ! { $ (# [$ variant_attr]) * pub struct $ member $ ($ rest) * } impl From <$ member > for $ name { fn from (e : $ member) -> $ name { $ name ::$ variant (e) } }) * generate_to_tokens ! { $ ($ remaining) * enum $ name { $ ($ variant [$ ($ rest) *] ,) * } }) }
};
}
