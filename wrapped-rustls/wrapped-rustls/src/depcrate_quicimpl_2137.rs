// Generated macro for impl_2137 (impl)
macro_rules! Depcrate_quicimpl_2137 {
() => {
// Module: crate::quic
// Provides: {"impl_2137"}
// Dependencies: {}
impl Keys { # [doc = " Construct keys for use with initial packets"] pub fn initial (version : Version , suite : & 'static Tls13CipherSuite , quic : & 'static dyn Algorithm , client_dst_connection_id : & [u8] , side : Side ,) -> Self { const CLIENT_LABEL : & [u8] = b"client in" ; const SERVER_LABEL : & [u8] = b"server in" ; let salt = version . initial_salt () ; let hs_secret = suite . hkdf_provider . extract_from_secret (Some (salt) , client_dst_connection_id) ; let secrets = Secrets { version , client : hkdf_expand_label_block (hs_secret . as_ref () , CLIENT_LABEL , & []) , server : hkdf_expand_label_block (hs_secret . as_ref () , SERVER_LABEL , & []) , suite , quic , side , } ; Self :: new (& secrets) } fn new (secrets : & Secrets) -> Self { let (local , remote) = secrets . local_remote () ; Self { local : DirectionalKeys :: new (secrets . suite , secrets . quic , local , secrets . version) , remote : DirectionalKeys :: new (secrets . suite , secrets . quic , remote , secrets . version) , } } }
};
}
