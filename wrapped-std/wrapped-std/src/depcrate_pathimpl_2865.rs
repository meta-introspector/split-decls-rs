// Generated macro for impl_2865 (impl)
macro_rules! Depcrate_pathimpl_2865 {
() => {
// Module: crate::path
// Provides: {"impl_2865"}
// Dependencies: {}
# [stable (feature = "cow_from_path" , since = "1.6.0")] impl < 'a > From < PathBuf > for Cow < 'a , Path > { # [doc = " Creates a clone-on-write pointer from an owned"] # [doc = " instance of [`PathBuf`]."] # [doc = ""] # [doc = " This conversion does not clone or allocate."] # [inline] fn from (s : PathBuf) -> Cow < 'a , Path > { Cow :: Owned (s) } }
};
}
