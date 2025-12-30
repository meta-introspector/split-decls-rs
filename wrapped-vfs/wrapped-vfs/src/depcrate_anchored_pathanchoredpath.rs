// Generated macro for AnchoredPath (struct)
macro_rules! Depcrate_anchored_pathAnchoredPath {
() => {
// Module: crate::anchored_path
// Provides: {"AnchoredPath"}
// Dependencies: {}
# [doc = " Path relative to a file."] # [doc = ""] # [doc = " Borrowed version of [`AnchoredPathBuf`]."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct AnchoredPath < 'a > { # [doc = " File that this path is relative to."] pub anchor : FileId , # [doc = " Path relative to `anchor`'s containing directory."] pub path : & 'a str , }
};
}
