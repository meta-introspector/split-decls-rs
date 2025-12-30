// Generated macro for impl_183 (impl)
macro_rules! Depcrate_thread_atomics_oneshotimpl_183 {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"impl_183"}
// Dependencies: {}
impl < T > Drop for Sender < T > { fn drop (& mut self) { # [allow (clippy :: significant_drop_in_scrutinee)] self . take_with (| shared , mut state | match state . deref () { State :: Waiting => { * state = State :: Dropped ; drop (state) ; shared . cvar . notify_one () ; shared . waker . wake () ; } State :: Taken | State :: Result (_) => unreachable ! ("left state intact after sending") , State :: Dropped => unreachable ! ("somehow dropped twice") , }) ; } }
};
}
