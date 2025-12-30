// Generated macro for impl_148 (impl)
macro_rules! Depcrate_http3_driver_serverimpl_148 {
() => {
// Module: crate::http3::driver::server
// Provides: {"impl_148"}
// Dependencies: {}
impl From < QuicCommand > for ServerH3Command { fn from (cmd : QuicCommand) -> Self { Self :: Core (H3Command :: QuicCmd (cmd)) } }
};
}
