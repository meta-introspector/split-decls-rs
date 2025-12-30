// Generated macro for impl_212 (impl)
macro_rules! Depcrate_http3_driver_test_utilsimpl_212 {
() => {
// Module: crate::http3::driver::test_utils
// Provides: {"impl_212"}
// Dependencies: {}
impl GetConnectionForHook for ClientHooks { fn qconn (pipe : & mut Pipe) -> & mut quiche :: Connection { & mut pipe . client } fn peer_qconn (pipe : & mut Pipe) -> & mut quiche :: Connection { & mut pipe . server } }
};
}
