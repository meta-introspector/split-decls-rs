// Generated macro for impl_184 (impl)
macro_rules! Depcrate_thread_atomics_oneshotimpl_184 {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"impl_184"}
// Dependencies: {}
impl < T > Sender < T > { # [doc = " Blocks or spinloops depending on support to get the inner [`State`]."] # [allow (clippy :: significant_drop_tightening)] fn take_with (& mut self , task : impl FnOnce (& Shared < T > , MutexGuard < '_ , State < T > >)) { if let Some (shared) = self . 0 . take () . and_then (| shared | shared . upgrade ()) { loop { let inner = match shared . value . try_lock () { Ok (inner) => inner , Err (TryLockError :: Poisoned (error)) => error . into_inner () , Err (TryLockError :: WouldBlock) => continue , } ; task (& shared , inner) ; break ; } } } # [doc = " Send `value` to [`Receiver`]."] pub (super) fn send (mut self , value : T) { self . take_with (move | shared , mut state | { * state = State :: Result (value) ; drop (state) ; shared . cvar . notify_one () ; shared . waker . wake () ; }) ; } }
};
}
