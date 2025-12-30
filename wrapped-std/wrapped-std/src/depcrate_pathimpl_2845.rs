// Generated macro for impl_2845 (impl)
macro_rules! Depcrate_pathimpl_2845 {
() => {
// Module: crate::path
// Provides: {"impl_2845"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Clone for PathBuf { # [inline] fn clone (& self) -> Self { PathBuf { inner : self . inner . clone () } } # [doc = " Clones the contents of `source` into `self`."] # [doc = ""] # [doc = " This method is preferred over simply assigning `source.clone()` to `self`,"] # [doc = " as it avoids reallocation if possible."] # [inline] fn clone_from (& mut self , source : & Self) { self . inner . clone_from (& source . inner) } }
};
}
