// Generated macro for impl_289 (impl)
macro_rules! Depcrate_http3_driverimpl_289 {
() => {
// Module: crate::http3::driver
// Provides: {"impl_289"}
// Dependencies: {}
impl From < quiche :: Error > for H3ConnectionError { fn from (err : quiche :: Error) -> Self { H3ConnectionError :: H3 (h3 :: Error :: TransportError (err)) } }
};
}
