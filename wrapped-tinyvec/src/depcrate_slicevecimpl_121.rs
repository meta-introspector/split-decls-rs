// Generated macro for impl_121 (impl)
macro_rules! Depcrate_slicevecimpl_121 {
() => {
// Module: crate::slicevec
// Provides: {"impl_121"}
// Dependencies: {}
impl < 's , T > Hash for SliceVec < 's , T > where T : Hash , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
};
}
