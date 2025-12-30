// Generated macro for impl_237 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_237 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_237"}
// Dependencies: {}
impl < 'a > From < & DnsName < 'a > > for ServerNamePayload < 'static > { fn from (value : & DnsName < 'a >) -> Self { Self :: SingleDnsName (trim_hostname_trailing_dot_for_sni (value)) } }
};
}
