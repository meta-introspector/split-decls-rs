// Generated macro for use_476 (pub_use)
macro_rules! Depcrate_fsuse_476 {
() => {
// Module: crate::fs
// Provides: {"use_476"}
// Dependencies: {}
# [doc = " Re-export types common to POSIX-ish platforms."] # [cfg (feature = "std")] # [cfg (unix)] pub use std :: os :: unix :: fs :: { DirEntryExt , FileExt , FileTypeExt , MetadataExt , OpenOptionsExt } ;
};
}
