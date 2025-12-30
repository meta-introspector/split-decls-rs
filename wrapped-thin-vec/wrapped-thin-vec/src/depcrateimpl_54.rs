// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'a , A , B > PartialEq < & 'a [B] > for ThinVec < A > where A : PartialEq < B > , { # [inline] fn eq (& self , other : & & 'a [B]) -> bool { self [..] == other [..] } }
};
}
