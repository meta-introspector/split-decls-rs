// Generated macro for FdSetIter (struct)
macro_rules! Depcrate_event_selectFdSetIter {
() => {
// Module: crate::event::select
// Provides: {"FdSetIter"}
// Dependencies: {}
# [doc = " An iterator over the fds in a set."] # [doc (alias = "FD_ISSET")] # [cfg (any (windows , target_os = "wasi"))] pub struct FdSetIter < 'a > { current : usize , fds : & 'a [FdSetElement] , }
};
}
