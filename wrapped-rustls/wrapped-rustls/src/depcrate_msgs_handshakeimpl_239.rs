// Generated macro for impl_239 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_239 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_239"}
// Dependencies: {}
impl HostNamePayload { fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { use pki_types :: ServerName ; let raw = PayloadU16 :: < NonEmpty > :: read (r) ? ; match ServerName :: try_from (raw . 0 . as_slice ()) { Ok (ServerName :: DnsName (d)) => Ok (Self :: HostName (d . to_owned ())) , Ok (ServerName :: IpAddress (_)) => Ok (Self :: IpAddress (raw)) , Ok (_) | Err (_) => Ok (Self :: Invalid (raw)) , } } }
};
}
