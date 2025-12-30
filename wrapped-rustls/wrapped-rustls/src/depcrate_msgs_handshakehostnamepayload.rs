// Generated macro for HostNamePayload (enum)
macro_rules! Depcrate_msgs_handshakeHostNamePayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"HostNamePayload"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) enum HostNamePayload { HostName (DnsName < 'static >) , IpAddress (PayloadU16 < NonEmpty >) , Invalid (PayloadU16 < NonEmpty >) , }
};
}
