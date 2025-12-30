// Generated macro for get_locks (function)
macro_rules! Depcrate_parallel_code_lockget_locks {
() => {
// Module: crate::parallel_code_lock
// Provides: {"get_locks"}
// Dependencies: {}
fn get_locks (names : Vec < & str >) -> Vec < crate :: code_lock :: UniqueReentrantMutex > { names . into_iter () . map (| name | { check_new_key (name) ; global_locks () . get (name) . expect ("key to be set") . get () . clone () }) . collect :: < Vec < _ > > () }
};
}
