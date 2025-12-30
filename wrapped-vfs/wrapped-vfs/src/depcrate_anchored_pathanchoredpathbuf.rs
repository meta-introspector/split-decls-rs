// Generated macro for AnchoredPathBuf (struct)
macro_rules! Depcrate_anchored_pathAnchoredPathBuf {
() => {
// Module: crate::anchored_path
// Provides: {"AnchoredPathBuf"}
// Dependencies: {}
# [doc = " Path relative to a file."] # [doc = ""] # [doc = " Owned version of [`AnchoredPath`]."] # [derive (Clone , PartialEq , Eq , Debug)] pub struct AnchoredPathBuf { # [doc = " File that this path is relative to."] pub anchor : FileId , # [doc = " Path relative to `anchor`'s containing directory."] pub path : String , }
};
}
