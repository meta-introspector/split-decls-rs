// Generated macro for impl_687 (impl)
macro_rules! Depcrate_ffi_os_strimpl_687 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_687"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToOwned for OsStr { type Owned = OsString ; # [inline] fn to_owned (& self) -> OsString { self . to_os_string () } # [inline] fn clone_into (& self , target : & mut OsString) { self . inner . clone_into (& mut target . inner) } }
};
}
