// Generated macro for init (function)
macro_rules! Depcrate_fs_inotifyinit {
() => {
// Module: crate::fs::inotify
// Provides: {"init"}
// Dependencies: {}
# [doc = " `inotify_init1(flags)`—Creates a new inotify object."] # [doc = ""] # [doc = " Use the [`CreateFlags::CLOEXEC`] flag to prevent the resulting file"] # [doc = " descriptor from being implicitly passed across `exec` boundaries."] # [doc (alias = "inotify_init1")] # [inline] pub fn init (flags : inotify :: CreateFlags) -> io :: Result < OwnedFd > { syscalls :: inotify_init1 (flags) }
};
}
