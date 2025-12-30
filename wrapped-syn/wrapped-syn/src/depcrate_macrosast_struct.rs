// Generated macro for ast_struct (macro)
macro_rules! Depcrate_macrosast_struct {
() => {
// Module: crate::macros
// Provides: {"ast_struct"}
// Dependencies: {}
macro_rules ! ast_struct { ($ (# [$ attr : meta]) * pub struct $ name : ident # full $ ($ rest : tt) *) => { # [cfg (feature = "full")] $ (# [$ attr]) * # [cfg_attr (feature = "extra-traits" , derive (Debug , Eq , PartialEq , Hash))] # [cfg_attr (feature = "clone-impls" , derive (Clone))] pub struct $ name $ ($ rest) * # [cfg (not (feature = "full"))] $ (# [$ attr]) * # [cfg_attr (feature = "extra-traits" , derive (Debug , Eq , PartialEq , Hash))] # [cfg_attr (feature = "clone-impls" , derive (Clone))] pub struct $ name { _noconstruct : () , } } ; ($ (# [$ attr : meta]) * pub struct $ name : ident $ ($ rest : tt) *) => { $ (# [$ attr]) * # [cfg_attr (feature = "extra-traits" , derive (Debug , Eq , PartialEq , Hash))] # [cfg_attr (feature = "clone-impls" , derive (Clone))] pub struct $ name $ ($ rest) * } ; }
};
}
