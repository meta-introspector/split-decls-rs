// Generated macro for impl_2870 (impl)
macro_rules! Depcrate_pathimpl_2870 {
() => {
// Module: crate::path
// Provides: {"impl_2870"}
// Dependencies: {}
# [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl From < & mut Path > for Arc < Path > { # [doc = " Converts a [`Path`] into an [`Arc`] by copying the [`Path`] data into a new [`Arc`] buffer."] # [inline] fn from (s : & mut Path) -> Arc < Path > { Arc :: from (& * s) } }
};
}
