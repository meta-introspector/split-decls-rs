// Generated macro for local_async_serial_core (function)
macro_rules! Depcrate_serial_code_locklocal_async_serial_core {
() => {
// Module: crate::serial_code_lock
// Provides: {"local_async_serial_core"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "async")] pub async fn local_async_serial_core (names : Vec < & str > , _path : Option < & str > , fut : impl std :: future :: Future < Output = () > ,) { core_internal ! (names) ; fut . await ; }
};
}
