// Generated macro for fs_serial_core_with_return (function)
macro_rules! Depcrate_serial_file_lockfs_serial_core_with_return {
() => {
// Module: crate::serial_file_lock
// Provides: {"fs_serial_core_with_return"}
// Dependencies: {}
# [doc (hidden)] pub fn fs_serial_core_with_return < E > (names : Vec < & str > , path : Option < & str > , function : fn () -> Result < () , E > ,) -> Result < () , E > { let mut locks = get_locks (& names , path) ; locks . iter_mut () . for_each (| lock | lock . start_serial ()) ; let res = panic :: catch_unwind (function) ; locks . into_iter () . for_each (| lock | lock . end_serial ()) ; match res { Ok (ret) => ret , Err (err) => { panic :: resume_unwind (err) ; } } }
};
}
