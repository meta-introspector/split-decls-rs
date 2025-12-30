// Generated macro for impl_2877 (impl)
macro_rules! Depcrate_pathimpl_2877 {
() => {
// Module: crate::path
// Provides: {"impl_2877"}
// Dependencies: {}
# [stable (feature = "eq_str_for_path" , since = "1.91.0")] impl cmp :: PartialEq < PathBuf > for str { # [inline] fn eq (& self , other : & PathBuf) -> bool { self == other . as_path () } }
};
}
