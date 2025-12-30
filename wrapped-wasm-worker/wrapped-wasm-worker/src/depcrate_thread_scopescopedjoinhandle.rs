// Generated macro for ScopedJoinHandle (struct)
macro_rules! Depcrate_thread_scopeScopedJoinHandle {
() => {
// Module: crate::thread::scope
// Provides: {"ScopedJoinHandle"}
// Dependencies: {}
# [doc = " See [`std::thread::ScopedJoinHandle`]."] pub struct ScopedJoinHandle < 'scope , T > { # [doc = " The underlying [`JoinHandle`]."] handle : JoinHandle < T > , # [doc = " Hold the `'scope` lifetime."] _scope : PhantomData < & 'scope () > , }
};
}
