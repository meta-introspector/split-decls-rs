// Generated macro for impl_645 (impl)
macro_rules! Depcrate_ffi_os_strimpl_645 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_645"}
// Dependencies: {}
# [stable (feature = "os_string_fmt_write" , since = "1.64.0")] impl fmt :: Write for OsString { fn write_str (& mut self , s : & str) -> fmt :: Result { self . push (s) ; Ok (()) } }
};
}
