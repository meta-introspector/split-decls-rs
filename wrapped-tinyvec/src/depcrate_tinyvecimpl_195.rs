// Generated macro for impl_195 (impl)
macro_rules! Depcrate_tinyvecimpl_195 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_195"}
// Dependencies: {}
impl < A : Array > Hash for TinyVec < A > where A :: Item : Hash , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
};
}
