// Generated macro for impl_1035 (impl)
macro_rules! Depcrate_timeimpl_1035 {
() => {
// Module: crate::time
// Provides: {"impl_1035"}
// Dependencies: {}
impl Hash for Time { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_u64 () . hash (state) } }
};
}
