// Generated macro for impl_2879 (impl)
macro_rules! Depcrate_pathimpl_2879 {
() => {
// Module: crate::path
// Provides: {"impl_2879"}
// Dependencies: {}
# [stable (feature = "eq_str_for_path" , since = "1.91.0")] impl cmp :: PartialEq < PathBuf > for String { # [inline] fn eq (& self , other : & PathBuf) -> bool { self . as_str () == other . as_path () } }
};
}
