// Generated macro for impl_2871 (impl)
macro_rules! Depcrate_pathimpl_2871 {
() => {
// Module: crate::path
// Provides: {"impl_2871"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < PathBuf > for Rc < Path > { # [doc = " Converts a [`PathBuf`] into an <code>[Rc]<[Path]></code> by moving the [`PathBuf`] data into"] # [doc = " a new [`Rc`] buffer."] # [inline] fn from (s : PathBuf) -> Rc < Path > { let rc : Rc < OsStr > = Rc :: from (s . into_os_string ()) ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const Path) } } }
};
}
