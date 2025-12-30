// Generated macro for use_2335 (pub_use)
macro_rules! Depcrate_fsuse_2335 {
() => {
// Module: crate::fs
// Provides: {"use_2335"}
// Dependencies: {}
# [doc = " Re-export types common to POSIX-ish platforms."] # [cfg (feature = "std")] # [cfg (unix)] pub use std :: os :: unix :: fs :: { DirEntryExt , FileExt , FileTypeExt , MetadataExt , OpenOptionsExt } ;
};
}
