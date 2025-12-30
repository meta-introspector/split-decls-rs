// Generated macro for impl_531 (impl)
macro_rules! Depcrate_small_c_strimpl_531 {
() => {
// Module: crate::small_c_str
// Provides: {"impl_531"}
// Dependencies: {}
impl Deref for SmallCStr { type Target = ffi :: CStr ; # [inline] fn deref (& self) -> & ffi :: CStr { self . as_c_str () } }
};
}
