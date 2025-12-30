// Generated macro for ast_enum (macro)
macro_rules! Depcrate_macrosast_enum {
() => {
// Module: crate::macros
// Provides: {"ast_enum"}
// Dependencies: {}
macro_rules ! ast_enum { ($ (# [$ enum_attr : meta]) * pub enum $ name : ident { $ ($ variants : tt) * }) => ($ (# [$ enum_attr]) * # [cfg_attr (feature = "extra-traits" , derive (Debug , Eq , PartialEq , Hash))] # [cfg_attr (feature = "clone-impls" , derive (Clone))] pub enum $ name { $ ($ variants) * }) }
};
}
