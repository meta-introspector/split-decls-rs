// Generated macro for GetConnectionForHook (trait)
macro_rules! Depcrate_http3_driver_test_utilsGetConnectionForHook {
() => {
// Module: crate::http3::driver::test_utils
// Provides: {"GetConnectionForHook"}
// Dependencies: {}
# [doc = " Helper trait to get the either right `quiche::Connection` for"] # [doc = " ourselvers (to use with the `H3Driver`) or our peer (to use"] # [doc = " with `quiche::H3::Connection`"] pub trait GetConnectionForHook { fn qconn (pipe : & mut Pipe) -> & mut quiche :: Connection ; fn peer_qconn (pipe : & mut Pipe) -> & mut quiche :: Connection ; }
};
}
