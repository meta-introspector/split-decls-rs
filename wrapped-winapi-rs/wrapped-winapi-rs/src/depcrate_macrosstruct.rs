// Generated macro for STRUCT (macro)
macro_rules! Depcrate_macrosSTRUCT {
() => {
// Module: crate::macros
// Provides: {"STRUCT"}
// Dependencies: {}
# [macro_export] macro_rules ! STRUCT { (# [debug] $ ($ rest : tt) *) => (STRUCT ! { # [cfg_attr (feature = "impl-debug" , derive (Debug))] $ ($ rest) * }) ; ($ (# [$ attrs : meta]) * struct $ name : ident { $ ($ field : ident : $ ftype : ty ,) + }) => (# [repr (C)] # [derive (Copy)] $ (# [$ attrs]) * pub struct $ name { $ (pub $ field : $ ftype ,) + } impl Clone for $ name { # [inline] fn clone (& self) -> $ name { * self } } # [cfg (feature = "impl-default")] impl Default for $ name { # [inline] fn default () -> $ name { unsafe { $ crate :: _core :: mem :: zeroed () } } }) ; }
};
}
