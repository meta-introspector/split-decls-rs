// Generated macro for ScopedJoinHandle (struct)
macro_rules! Depcrate_thread_scopedScopedJoinHandle {
() => {
// Module: crate::thread::scoped
// Provides: {"ScopedJoinHandle"}
// Dependencies: {}
# [doc = " An owned permission to join on a scoped thread (block on its termination)."] # [doc = ""] # [doc = " See [`Scope::spawn`] for details."] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub struct ScopedJoinHandle < 'scope , T > (JoinInner < 'scope , T >) ;
};
}
