// Generated macro for ClientHooks (struct)
macro_rules! Depcrate_http3_driver_clientClientHooks {
() => {
// Module: crate::http3::driver::client
// Provides: {"ClientHooks"}
// Dependencies: {}
pub struct ClientHooks { # [doc = " Mapping from stream IDs to the associated [`PendingClientRequest`]."] pending_requests : BTreeMap < u64 , PendingClientRequest > , }
};
}
