// Generated macro for fs_async_serial_core (function)
macro_rules! Depcrate_serial_file_lockfs_async_serial_core {
() => {
// Module: crate::serial_file_lock
// Provides: {"fs_async_serial_core"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "async")] pub async fn fs_async_serial_core (names : Vec < & str > , path : Option < & str > , fut : impl std :: future :: Future < Output = () > ,) { let mut locks = get_locks (& names , path) ; locks . iter_mut () . for_each (| lock | lock . start_serial ()) ; fut . await ; locks . into_iter () . for_each (| lock | lock . end_serial ()) ; }
};
}
