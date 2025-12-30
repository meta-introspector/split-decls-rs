// Generated macro for impl_476 (impl)
macro_rules! Depcrate_msgs_persistimpl_476 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_476"}
// Dependencies: {}
impl Codec < '_ > for CommonServerSessionValue { fn encode (& self , bytes : & mut Vec < u8 >) { if let Some (sni) = & self . sni { 1u8 . encode (bytes) ; let sni_bytes : & str = sni . as_ref () ; PayloadU8 :: < MaybeEmpty > :: encode_slice (sni_bytes . as_bytes () , bytes) ; } else { 0u8 . encode (bytes) ; } self . cipher_suite . encode (bytes) ; if let Some (identity) = & self . peer_identity { 1u8 . encode (bytes) ; identity . encode (bytes) ; } else { 0u8 . encode (bytes) ; } if let Some (alpn) = & self . alpn { 1u8 . encode (bytes) ; alpn . encode (bytes) ; } else { 0u8 . encode (bytes) ; } self . application_data . encode (bytes) ; self . creation_time_sec . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let sni = match u8 :: read (r) ? { 1 => { let dns_name = PayloadU8 :: < MaybeEmpty > :: read (r) ? ; let dns_name = match DnsName :: try_from (dns_name . 0 . as_slice ()) { Ok (dns_name) => dns_name . to_owned () , Err (_) => return Err (InvalidMessage :: InvalidServerName) , } ; Some (dns_name) } _ => None , } ; Ok (Self { sni , cipher_suite : CipherSuite :: read (r) ? , peer_identity : match u8 :: read (r) ? { 1 => Some (Identity :: read (r) ? . into_owned ()) , _ => None , } , alpn : match u8 :: read (r) ? { 1 => Some (ProtocolName :: read (r) ?) , _ => None , } , application_data : PayloadU16 :: read (r) ? , creation_time_sec : u64 :: read (r) ? , }) } }
};
}
