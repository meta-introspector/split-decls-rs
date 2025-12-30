// Generated macro for impl_235 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_235 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_235"}
// Dependencies: {}
impl ServerNamePayload < '_ > { fn into_owned (self) -> ServerNamePayload < 'static > { match self { Self :: SingleDnsName (d) => ServerNamePayload :: SingleDnsName (d . to_owned ()) , Self :: IpAddress => ServerNamePayload :: IpAddress , Self :: Invalid => ServerNamePayload :: Invalid , } } # [doc = " RFC6066: `ServerName server_name_list<1..2^16-1>`"] const SIZE_LEN : ListLength = ListLength :: NonZeroU16 { empty_error : InvalidMessage :: IllegalEmptyList ("ServerNames") , } ; # [doc = " Get the `DnsName` out of this `ServerNamePayload` if it contains one."] # [doc = " The returned `DnsName` will be normalized (converted to lowercase)."] pub (crate) fn to_dns_name_normalized (& self) -> Option < DnsName < 'static > > { match self { Self :: SingleDnsName (dns_name) => Some (dns_name . to_lowercase_owned ()) , Self :: IpAddress => None , Self :: Invalid => None , } } }
};
}
