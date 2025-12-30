// Generated macro for local_parallel_core (function)
macro_rules! Depcrate_parallel_code_locklocal_parallel_core {
() => {
// Module: crate::parallel_code_lock
// Provides: {"local_parallel_core"}
// Dependencies: {}
# [doc (hidden)] pub fn local_parallel_core (names : Vec < & str > , _path : Option < & str > , function : fn ()) { let locks = get_locks (names) ; locks . iter () . for_each (| lock | lock . start_parallel ()) ; let res = panic :: catch_unwind (| | { function () ; }) ; locks . iter () . for_each (| lock | lock . end_parallel ()) ; if let Err (err) = res { panic :: resume_unwind (err) ; } }
};
}
