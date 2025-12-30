// Generated macro for fs_parallel_core (function)
macro_rules! Depcrate_parallel_file_lockfs_parallel_core {
() => {
// Module: crate::parallel_file_lock
// Provides: {"fs_parallel_core"}
// Dependencies: {}
# [doc (hidden)] pub fn fs_parallel_core (names : Vec < & str > , path : Option < & str > , function : fn ()) { get_locks (& names , path) . iter_mut () . for_each (| lock | lock . start_parallel ()) ; let res = panic :: catch_unwind (| | { function () ; }) ; get_locks (& names , path) . into_iter () . for_each (| lock | lock . end_parallel ()) ; if let Err (err) = res { panic :: resume_unwind (err) ; } }
};
}
