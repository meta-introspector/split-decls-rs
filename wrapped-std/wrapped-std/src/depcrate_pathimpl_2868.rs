// Generated macro for impl_2868 (impl)
macro_rules! Depcrate_pathimpl_2868 {
() => {
// Module: crate::path
// Provides: {"impl_2868"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < PathBuf > for Arc < Path > { # [doc = " Converts a [`PathBuf`] into an <code>[Arc]<[Path]></code> by moving the [`PathBuf`] data"] # [doc = " into a new [`Arc`] buffer."] # [inline] fn from (s : PathBuf) -> Arc < Path > { let arc : Arc < OsStr > = Arc :: from (s . into_os_string ()) ; unsafe { Arc :: from_raw (Arc :: into_raw (arc) as * const Path) } } }
};
}
