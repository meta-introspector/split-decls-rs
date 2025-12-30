// Generated macro for make_server_config_with_optional_client_auth (function)
macro_rules! Depcratemake_server_config_with_optional_client_auth {
() => {
// Module: crate
// Provides: {"make_server_config_with_optional_client_auth"}
// Dependencies: {}
pub fn make_server_config_with_optional_client_auth (kt : KeyType , crls : Vec < CertificateRevocationListDer < 'static > > , provider : & CryptoProvider ,) -> ServerConfig { make_server_config_with_client_verifier (kt , webpki_client_verifier_builder (kt . client_root_store () , provider) . with_crls (crls) . allow_unknown_revocation_status () . allow_unauthenticated () , provider ,) }
};
}
