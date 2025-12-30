// Generated macro for impl_2869 (impl)
macro_rules! Depcrate_pathimpl_2869 {
() => {
// Module: crate::path
// Provides: {"impl_2869"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < & Path > for Arc < Path > { # [doc = " Converts a [`Path`] into an [`Arc`] by copying the [`Path`] data into a new [`Arc`] buffer."] # [inline] fn from (s : & Path) -> Arc < Path > { let arc : Arc < OsStr > = Arc :: from (s . as_os_str ()) ; unsafe { Arc :: from_raw (Arc :: into_raw (arc) as * const Path) } } }
};
}
