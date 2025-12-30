// Generated macro for impl_75 (impl)
macro_rules! Depcrate_testsimpl_75 {
() => {
// Module: crate::tests
// Provides: {"impl_75"}
// Dependencies: {}
impl Hash for Bytes < '_ > { # [allow (unused_must_use)] fn hash < H : Hasher > (& self , state : & mut H) { let Bytes (v) = * self ; state . write (v) ; } }
};
}
