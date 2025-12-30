// Generated macro for impl_2878 (impl)
macro_rules! Depcrate_pathimpl_2878 {
() => {
// Module: crate::path
// Provides: {"impl_2878"}
// Dependencies: {}
# [stable (feature = "eq_str_for_path" , since = "1.91.0")] impl cmp :: PartialEq < String > for PathBuf { # [inline] fn eq (& self , other : & String) -> bool { self . as_path () == other . as_str () } }
};
}
