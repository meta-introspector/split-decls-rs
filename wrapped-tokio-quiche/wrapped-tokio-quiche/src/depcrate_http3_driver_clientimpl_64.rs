// Generated macro for impl_64 (impl)
macro_rules! Depcrate_http3_driver_clientimpl_64 {
() => {
// Module: crate::http3::driver::client
// Provides: {"impl_64"}
// Dependencies: {}
impl From < QuicCommand > for ClientH3Command { fn from (cmd : QuicCommand) -> Self { Self :: Core (H3Command :: QuicCmd (cmd)) } }
};
}
