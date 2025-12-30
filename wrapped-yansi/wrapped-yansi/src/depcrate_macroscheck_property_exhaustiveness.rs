// Generated macro for check_property_exhaustiveness (macro)
macro_rules! Depcrate_macroscheck_property_exhaustiveness {
() => {
// Module: crate::macros
// Provides: {"check_property_exhaustiveness"}
// Dependencies: {}
macro_rules ! check_property_exhaustiveness { ($ A : ident $ ({ $ ($ (# [$ pattr : meta]) * $ p : ident => $ V : path $ ([$ ($ a : tt) *]) ?) ,* $ (,) ? }) ?) => { const _ : () = { $ (use crate ::*; fn _check () { # [allow (unreachable_code)] match { let _v : $ A = todo ! () ; _v } { $ ($ V { .. } => { } ,) * } }) ? } ; } }
};
}
