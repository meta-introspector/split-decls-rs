// Generated macro for array_impls (macro)
macro_rules! Depcratearray_impls {
() => {
// Module: crate
// Provides: {"array_impls"}
// Dependencies: {}
macro_rules ! array_impls { ($ ($ N : expr) *) => { $ (impl < A , B > PartialEq < [B ; $ N] > for ThinVec < A > where A : PartialEq < B > { # [inline] fn eq (& self , other : & [B ; $ N]) -> bool { self [..] == other [..] } } impl <'a , A , B > PartialEq <&'a [B ; $ N] > for ThinVec < A > where A : PartialEq < B > { # [inline] fn eq (& self , other : &&'a [B ; $ N]) -> bool { self [..] == other [..] } }) * } }
};
}
