// Generated macro for impl_2818 (impl)
macro_rules! Depcrate_pathimpl_2818 {
() => {
// Module: crate::path
// Provides: {"impl_2818"}
// Dependencies: {}
# [stable (feature = "path_component_asref" , since = "1.25.0")] impl AsRef < Path > for Component < '_ > { # [inline] fn as_ref (& self) -> & Path { self . as_os_str () . as_ref () } }
};
}
