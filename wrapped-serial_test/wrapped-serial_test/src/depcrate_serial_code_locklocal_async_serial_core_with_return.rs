// Generated macro for local_async_serial_core_with_return (function)
macro_rules! Depcrate_serial_code_locklocal_async_serial_core_with_return {
() => {
// Module: crate::serial_code_lock
// Provides: {"local_async_serial_core_with_return"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "async")] pub async fn local_async_serial_core_with_return < R , E > (names : Vec < & str > , _path : Option < & str > , fut : impl std :: future :: Future < Output = Result < R , E > > + std :: marker :: Send ,) -> Result < R , E > { core_internal ! (names) ; fut . await }
};
}
