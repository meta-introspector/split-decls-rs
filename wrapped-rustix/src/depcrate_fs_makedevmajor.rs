// Generated macro for major (function)
macro_rules! Depcrate_fs_makedevmajor {
() => {
// Module: crate::fs::makedev
// Provides: {"major"}
// Dependencies: {}
# [doc = " `major(dev)`—Compute the major ID of a given device ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/major.3.html"] # [inline] pub fn major (dev : Dev) -> u32 { backend :: fs :: makedev :: major (dev) }
};
}
