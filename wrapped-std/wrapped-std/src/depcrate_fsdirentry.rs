// Generated macro for DirEntry (struct)
macro_rules! Depcrate_fsDirEntry {
() => {
// Module: crate::fs
// Provides: {"DirEntry"}
// Dependencies: {}
# [doc = " Entries returned by the [`ReadDir`] iterator."] # [doc = ""] # [doc = " An instance of `DirEntry` represents an entry inside of a directory on the"] # [doc = " filesystem. Each entry can be inspected via methods to learn about the full"] # [doc = " path or possibly other metadata through per-platform extension traits."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " On Unix, the `DirEntry` struct contains an internal reference to the open"] # [doc = " directory. Holding `DirEntry` objects will consume a file handle even"] # [doc = " after the `ReadDir` iterator is dropped."] # [doc = ""] # [doc = " Note that this [may change in the future][changes]."] # [doc = ""] # [doc = " [changes]: io#platform-specific-behavior"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct DirEntry (fs_imp :: DirEntry) ;
};
}
