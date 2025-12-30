// Generated macro for impl_652 (impl)
macro_rules! Depcrate_ffi_os_strimpl_652 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_652"}
// Dependencies: {}
# [stable (feature = "more_box_slice_clone" , since = "1.29.0")] impl Clone for Box < OsStr > { # [inline] fn clone (& self) -> Self { self . to_os_string () . into_boxed_os_str () } }
};
}
