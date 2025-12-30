// Generated macro for ServerNamePayload (enum)
macro_rules! Depcrate_msgs_handshakeServerNamePayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"ServerNamePayload"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) enum ServerNamePayload < 'a > { # [doc = " A successfully decoded value:"] SingleDnsName (DnsName < 'a >) , # [doc = " A DNS name which was actually an IP address"] IpAddress , # [doc = " A successfully decoded, but syntactically-invalid value."] Invalid , }
};
}
