// Generated macro for impl_477 (impl)
macro_rules! Depcrate_thread_yield_nowimpl_477 {
() => {
// Module: crate::thread::yield_now
// Provides: {"impl_477"}
// Dependencies: {}
impl Drop for YieldNowFuture { fn drop (& mut self) { if let Some (state) = self . 0 . take () { match state { State :: Scheduler { controller , .. } => controller . abort () , State :: Idle { handle , .. } => Global :: with (| global | { let Global :: Window (window) = global else { unreachable ! ("expected `Window`") } ; window . cancel_idle_callback (handle) ; }) , State :: Channel { port , .. } => { port . set_onmessage (None) ; port . close () ; } State :: None => () , } } } }
};
}
