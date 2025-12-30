// Generated macro for impl_213 (impl)
macro_rules! Depcrate_http3_driver_test_utilsimpl_213 {
() => {
// Module: crate::http3::driver::test_utils
// Provides: {"impl_213"}
// Dependencies: {}
impl GetConnectionForHook for ServerHooks { fn qconn (pipe : & mut Pipe) -> & mut quiche :: Connection { & mut pipe . server } fn peer_qconn (pipe : & mut Pipe) -> & mut quiche :: Connection { & mut pipe . client } }
};
}
