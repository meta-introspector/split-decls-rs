// Generated macro for quiche_config_with_tls (function)
macro_rules! Depcrate_settings_configquiche_config_with_tls {
() => {
// Module: crate::settings::config
// Provides: {"quiche_config_with_tls"}
// Dependencies: {}
fn quiche_config_with_tls (tls_cert : Option < TlsCertificatePaths > ,) -> QuicResult < quiche :: Config > { let Some (tls) = tls_cert else { return Ok (quiche :: Config :: new (quiche :: PROTOCOL_VERSION) . unwrap ()) ; } ; match tls . kind { # [cfg (not (feature = "rpk"))] CertificateKind :: RawPublicKey => { panic ! ("Can't use RPK when compiled without rpk feature") ; } , # [cfg (feature = "rpk")] CertificateKind :: RawPublicKey => { let mut ssl_ctx_builder = boring :: ssl :: SslContextBuilder :: new_rpk () ? ; let raw_public_key = read_file (tls . cert) ? ; ssl_ctx_builder . set_rpk_certificate (& raw_public_key) ? ; let raw_private_key = read_file (tls . private_key) ? ; let pkey = boring :: pkey :: PKey :: private_key_from_pem (& raw_private_key) ? ; ssl_ctx_builder . set_null_chain_private_key (& pkey) ? ; Ok (quiche :: Config :: with_boring_ssl_ctx_builder (quiche :: PROTOCOL_VERSION , ssl_ctx_builder ,) ?) } , CertificateKind :: X509 => { let mut config = quiche :: Config :: new (quiche :: PROTOCOL_VERSION) . unwrap () ; config . load_cert_chain_from_pem_file (tls . cert) ? ; config . load_priv_key_from_pem_file (tls . private_key) ? ; Ok (config) } , } }
};
}
