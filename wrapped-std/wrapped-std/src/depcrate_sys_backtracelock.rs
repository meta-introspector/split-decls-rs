// Generated macro for lock (function)
macro_rules! Depcrate_sys_backtracelock {
() => {
// Module: crate::sys::backtrace
// Provides: {"lock"}
// Dependencies: {}
pub (crate) fn lock < 'a > () -> BacktraceLock < 'a > { static LOCK : Mutex < () > = Mutex :: new (()) ; BacktraceLock (LOCK . lock () . unwrap_or_else (PoisonError :: into_inner)) }
};
}
