// Generated macro for impl_2864 (impl)
macro_rules! Depcrate_pathimpl_2864 {
() => {
// Module: crate::path
// Provides: {"impl_2864"}
// Dependencies: {}
# [stable (feature = "cow_from_path" , since = "1.6.0")] impl < 'a > From < & 'a Path > for Cow < 'a , Path > { # [doc = " Creates a clone-on-write pointer from a reference to"] # [doc = " [`Path`]."] # [doc = ""] # [doc = " This conversion does not clone or allocate."] # [inline] fn from (s : & 'a Path) -> Cow < 'a , Path > { Cow :: Borrowed (s) } }
};
}
