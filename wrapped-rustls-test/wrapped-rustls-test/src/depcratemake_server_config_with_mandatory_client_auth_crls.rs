// Generated macro for make_server_config_with_mandatory_client_auth_crls (function)
macro_rules! Depcratemake_server_config_with_mandatory_client_auth_crls {
() => {
// Module: crate
// Provides: {"make_server_config_with_mandatory_client_auth_crls"}
// Dependencies: {}
pub fn make_server_config_with_mandatory_client_auth_crls (kt : KeyType , crls : Vec < CertificateRevocationListDer < 'static > > , provider : & CryptoProvider ,) -> ServerConfig { make_server_config_with_client_verifier (kt , webpki_client_verifier_builder (kt . client_root_store () , provider) . with_crls (crls) , provider ,) }
};
}
