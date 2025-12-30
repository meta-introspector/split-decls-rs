// Generated macro for cleanup (function)
macro_rules! Depcrate_io_stdiocleanup {
() => {
// Module: crate::io::stdio
// Provides: {"cleanup"}
// Dependencies: {}
pub fn cleanup () { let mut initialized = false ; let stdout = STDOUT . get_or_init (| | { initialized = true ; ReentrantLock :: new (RefCell :: new (LineWriter :: with_capacity (0 , stdout_raw ()))) }) ; if ! initialized { if let Some (lock) = stdout . try_lock () { * lock . borrow_mut () = LineWriter :: with_capacity (0 , stdout_raw ()) ; } } }
};
}
