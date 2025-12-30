// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < T > Hash for ThinVec < T > where T : Hash , { fn hash < H > (& self , state : & mut H) where H : Hasher , { self [..] . hash (state) ; } }
};
}
