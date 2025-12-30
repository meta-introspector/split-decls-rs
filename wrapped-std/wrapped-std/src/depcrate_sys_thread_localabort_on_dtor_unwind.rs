// Generated macro for abort_on_dtor_unwind (function)
macro_rules! Depcrate_sys_thread_localabort_on_dtor_unwind {
() => {
// Module: crate::sys::thread_local
// Provides: {"abort_on_dtor_unwind"}
// Dependencies: {}
# [doc = " Run a callback in a scenario which must not unwind (such as a `extern \"C\""] # [doc = " fn` declared in a user crate). If the callback unwinds anyway, then"] # [doc = " `rtabort` with a message about thread local panicking on drop."] # [inline] # [allow (dead_code)] fn abort_on_dtor_unwind (f : impl FnOnce ()) { let guard = DtorUnwindGuard ; f () ; core :: mem :: forget (guard) ; struct DtorUnwindGuard ; impl Drop for DtorUnwindGuard { # [inline] fn drop (& mut self) { rtabort ! ("thread local panicked on drop") ; } } }
};
}
