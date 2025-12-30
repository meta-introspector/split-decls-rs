// Generated macro for lock (function)
macro_rules! Depcrate_termlock {
() => {
// Module: crate::term
// Provides: {"lock"}
// Dependencies: {}
pub (crate) fn lock () -> MutexGuard < 'static , Term > { TERM . get_or_init (| | Mutex :: new (Term :: new ())) . lock () . unwrap_or_else (PoisonError :: into_inner) }
};
}
