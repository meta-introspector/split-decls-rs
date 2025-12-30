// Generated macro for global_locks (function)
macro_rules! Depcrate_code_lockglobal_locks {
() => {
// Module: crate::code_lock
// Provides: {"global_locks"}
// Dependencies: {}
# [inline] pub (crate) fn global_locks () -> & 'static HashMap < String , UniqueReentrantMutex > { # [cfg (feature = "test_logging")] let _ = env_logger :: builder () . try_init () ; static LOCKS : OnceCell < HashMap < String , UniqueReentrantMutex > > = OnceCell :: new () ; LOCKS . get_or_init (HashMap :: new) }
};
}
