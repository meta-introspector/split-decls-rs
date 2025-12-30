// Generated macro for impl_1109 (impl)
macro_rules! Depcrate_utc_offsetimpl_1109 {
() => {
// Module: crate::utc_offset
// Provides: {"impl_1109"}
// Dependencies: {}
impl Hash for UtcOffset { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { state . write_u32 (self . as_u32 ()) ; } }
};
}
