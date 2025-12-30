// Generated macro for impl_292 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_292 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_292"}
// Dependencies: {}
impl ClientHelloPayload { pub (crate) fn ech_inner_encoding (& self , to_compress : Vec < ExtensionType >) -> Vec < u8 > { let mut bytes = Vec :: new () ; self . payload_encode (& mut bytes , Encoding :: EchInnerHello { to_compress }) ; bytes } pub (crate) fn payload_encode (& self , bytes : & mut Vec < u8 > , purpose : Encoding) { self . client_version . encode (bytes) ; self . random . encode (bytes) ; match purpose { Encoding :: EchInnerHello { .. } => SessionId :: empty () . encode (bytes) , _ => self . session_id . encode (bytes) , } self . cipher_suites . encode (bytes) ; self . compression_methods . encode (bytes) ; let to_compress = match purpose { Encoding :: EchInnerHello { to_compress } if ! to_compress . is_empty () => to_compress , _ => { self . extensions . encode (bytes) ; return ; } } ; let mut compressed = self . extensions . clone () ; for e in & to_compress { compressed . clear (* e) ; } compressed . encrypted_client_hello_outer = Some (to_compress) ; compressed . encode (bytes) ; } pub (crate) fn has_keyshare_extension_with_duplicates (& self) -> bool { self . key_shares . as_ref () . map (| entries | { has_duplicates :: < _ , _ , u16 > (entries . iter () . map (| kse | u16 :: from (kse . group)) ,) }) . unwrap_or_default () } pub (crate) fn has_certificate_compression_extension_with_duplicates (& self) -> bool { if let Some (algs) = & self . certificate_compression_algorithms { has_duplicates :: < _ , _ , u16 > (algs . iter () . copied ()) } else { false } } }
};
}
