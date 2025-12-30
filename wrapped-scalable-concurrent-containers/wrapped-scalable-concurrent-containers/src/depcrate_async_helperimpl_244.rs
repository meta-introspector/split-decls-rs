// Generated macro for impl_244 (impl)
macro_rules! Depcrate_async_helperimpl_244 {
() => {
// Module: crate::async_helper
// Provides: {"impl_244"}
// Dependencies: {}
impl TryWait for () { # [inline] fn try_wait (& mut self , lock : & Lock) { let mut pinned_pager = pin ! (Pager :: default ()) ; lock . register_pager (& mut pinned_pager , Mode :: WaitExclusive , true) ; let _ : Result < _ , _ > = pinned_pager . poll_sync () ; } }
};
}
