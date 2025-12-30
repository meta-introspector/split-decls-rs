// Generated macro for impl_633 (impl)
macro_rules! Depcrate_ffi_os_strimpl_633 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_633"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Clone for OsString { # [inline] fn clone (& self) -> Self { OsString { inner : self . inner . clone () } } # [doc = " Clones the contents of `source` into `self`."] # [doc = ""] # [doc = " This method is preferred over simply assigning `source.clone()` to `self`,"] # [doc = " as it avoids reallocation if possible."] # [inline] fn clone_from (& mut self , source : & Self) { self . inner . clone_from (& source . inner) } }
};
}
