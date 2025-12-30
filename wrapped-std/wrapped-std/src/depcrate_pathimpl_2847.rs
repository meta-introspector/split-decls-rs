// Generated macro for impl_2847 (impl)
macro_rules! Depcrate_pathimpl_2847 {
() => {
// Module: crate::path
// Provides: {"impl_2847"}
// Dependencies: {}
# [stable (feature = "box_from_mut_slice" , since = "1.84.0")] impl From < & mut Path > for Box < Path > { # [doc = " Creates a boxed [`Path`] from a reference."] # [doc = ""] # [doc = " This will allocate and clone `path` to it."] fn from (path : & mut Path) -> Box < Path > { Self :: from (& * path) } }
};
}
