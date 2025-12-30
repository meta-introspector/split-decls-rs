// Generated macro for impl_2851 (impl)
macro_rules! Depcrate_pathimpl_2851 {
() => {
// Module: crate::path
// Provides: {"impl_2851"}
// Dependencies: {}
# [stable (feature = "more_box_slice_clone" , since = "1.29.0")] impl Clone for Box < Path > { # [inline] fn clone (& self) -> Self { self . to_path_buf () . into_boxed_path () } }
};
}
