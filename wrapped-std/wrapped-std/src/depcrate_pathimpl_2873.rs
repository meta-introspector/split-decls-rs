// Generated macro for impl_2873 (impl)
macro_rules! Depcrate_pathimpl_2873 {
() => {
// Module: crate::path
// Provides: {"impl_2873"}
// Dependencies: {}
# [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl From < & mut Path > for Rc < Path > { # [doc = " Converts a [`Path`] into an [`Rc`] by copying the [`Path`] data into a new [`Rc`] buffer."] # [inline] fn from (s : & mut Path) -> Rc < Path > { Rc :: from (& * s) } }
};
}
