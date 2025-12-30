// Generated macro for impl_2906 (impl)
macro_rules! Depcrate_pathimpl_2906 {
() => {
// Module: crate::path
// Provides: {"impl_2906"}
// Dependencies: {}
# [stable (feature = "cow_os_str_as_ref_path" , since = "1.8.0")] impl AsRef < Path > for Cow < '_ , OsStr > { # [inline] fn as_ref (& self) -> & Path { Path :: new (self) } }
};
}
