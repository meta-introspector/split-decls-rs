// Generated macro for impl_66 (impl)
macro_rules! Depcrate_arrayvecimpl_66 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_66"}
// Dependencies: {}
impl < A : Array > Hash for ArrayVec < A > where A :: Item : Hash , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
};
}
