// Generated macro for impl_1240 (impl)
macro_rules! Depcrate_ioimpl_1240 {
() => {
// Module: crate::io
// Provides: {"impl_1240"}
// Dependencies: {}
# [stable (feature = "iovec" , since = "1.36.0")] impl < 'a > Deref for IoSlice < 'a > { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { self . 0 . as_slice () } }
};
}
