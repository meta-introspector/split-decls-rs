// Generated macro for impl_2872 (impl)
macro_rules! Depcrate_pathimpl_2872 {
() => {
// Module: crate::path
// Provides: {"impl_2872"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < & Path > for Rc < Path > { # [doc = " Converts a [`Path`] into an [`Rc`] by copying the [`Path`] data into a new [`Rc`] buffer."] # [inline] fn from (s : & Path) -> Rc < Path > { let rc : Rc < OsStr > = Rc :: from (s . as_os_str ()) ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const Path) } } }
};
}
