// Generated macro for impl_2867 (impl)
macro_rules! Depcrate_pathimpl_2867 {
() => {
// Module: crate::path
// Provides: {"impl_2867"}
// Dependencies: {}
# [stable (feature = "pathbuf_from_cow_path" , since = "1.28.0")] impl < 'a > From < Cow < 'a , Path > > for PathBuf { # [doc = " Converts a clone-on-write pointer to an owned path."] # [doc = ""] # [doc = " Converting from a `Cow::Owned` does not clone or allocate."] # [inline] fn from (p : Cow < 'a , Path >) -> Self { p . into_owned () } }
};
}
