// Generated macro for fs_serial_core (function)
macro_rules! Depcrate_serial_file_lockfs_serial_core {
() => {
// Module: crate::serial_file_lock
// Provides: {"fs_serial_core"}
// Dependencies: {}
# [doc (hidden)] pub fn fs_serial_core (names : Vec < & str > , path : Option < & str > , function : fn ()) { assert ! (names . len () > 0) ; let mut locks = get_locks (& names , path) ; locks . iter_mut () . for_each (| lock | lock . start_serial ()) ; let res = panic :: catch_unwind (function) ; locks . into_iter () . for_each (| lock | lock . end_serial ()) ; if let Err (err) = res { panic :: resume_unwind (err) ; } }
};
}
