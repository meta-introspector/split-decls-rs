// Generated macro for impl_2846 (impl)
macro_rules! Depcrate_pathimpl_2846 {
() => {
// Module: crate::path
// Provides: {"impl_2846"}
// Dependencies: {}
# [stable (feature = "box_from_path" , since = "1.17.0")] impl From < & Path > for Box < Path > { # [doc = " Creates a boxed [`Path`] from a reference."] # [doc = ""] # [doc = " This will allocate and clone `path` to it."] fn from (path : & Path) -> Box < Path > { let boxed : Box < OsStr > = path . inner . into () ; let rw = Box :: into_raw (boxed) as * mut Path ; unsafe { Box :: from_raw (rw) } } }
};
}
