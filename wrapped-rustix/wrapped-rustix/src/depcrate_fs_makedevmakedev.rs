// Generated macro for makedev (function)
macro_rules! Depcrate_fs_makedevmakedev {
() => {
// Module: crate::fs::makedev
// Provides: {"makedev"}
// Dependencies: {}
# [doc = " `makedev(maj, min)`—Compute a device ID from a given major and minor ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/makedev.3.html"] # [inline] pub fn makedev (maj : u32 , min : u32) -> Dev { backend :: fs :: makedev :: makedev (maj , min) }
};
}
