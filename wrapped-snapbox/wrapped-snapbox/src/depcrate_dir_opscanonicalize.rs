// Generated macro for canonicalize (function)
macro_rules! Depcrate_dir_opscanonicalize {
() => {
// Module: crate::dir::ops
// Provides: {"canonicalize"}
// Dependencies: {}
pub (crate) fn canonicalize (path : & std :: path :: Path) -> Result < std :: path :: PathBuf , std :: io :: Error > { # [cfg (feature = "dir")] { dunce :: canonicalize (path) } # [cfg (not (feature = "dir"))] { Ok (strip_trailing_slash (path) . to_owned ()) } }
};
}
