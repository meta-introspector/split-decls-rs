// Generated macro for remove_watch (function)
macro_rules! Depcrate_fs_inotifyremove_watch {
() => {
// Module: crate::fs::inotify
// Provides: {"remove_watch"}
// Dependencies: {}
# [doc = " `inotify_rm_watch(self, wd)`—Removes a watch from this inotify."] # [doc = ""] # [doc = " The watch descriptor provided should have previously been returned by"] # [doc = " [`inotify::add_watch`] and not previously have been removed."] # [doc (alias = "inotify_rm_watch")] # [inline] pub fn remove_watch < Fd : AsFd > (inot : Fd , wd : i32) -> io :: Result < () > { syscalls :: inotify_rm_watch (inot . as_fd () , wd) }
};
}
