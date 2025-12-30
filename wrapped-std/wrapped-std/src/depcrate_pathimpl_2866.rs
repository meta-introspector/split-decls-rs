// Generated macro for impl_2866 (impl)
macro_rules! Depcrate_pathimpl_2866 {
() => {
// Module: crate::path
// Provides: {"impl_2866"}
// Dependencies: {}
# [stable (feature = "cow_from_pathbuf_ref" , since = "1.28.0")] impl < 'a > From < & 'a PathBuf > for Cow < 'a , Path > { # [doc = " Creates a clone-on-write pointer from a reference to"] # [doc = " [`PathBuf`]."] # [doc = ""] # [doc = " This conversion does not clone or allocate."] # [inline] fn from (p : & 'a PathBuf) -> Cow < 'a , Path > { Cow :: Borrowed (p . as_path ()) } }
};
}
