// Generated macro for make_disjoint_suite_configs (function)
macro_rules! Depcratemake_disjoint_suite_configs {
() => {
// Module: crate
// Provides: {"make_disjoint_suite_configs"}
// Dependencies: {}
# [doc = " Return a client and server config that don't share a common cipher suite"] pub fn make_disjoint_suite_configs (provider : CryptoProvider) -> (ClientConfig , ServerConfig) { let kt = KeyType :: Rsa2048 ; let client_provider = CryptoProvider { tls13_cipher_suites : provider . tls13_cipher_suites . iter () . cloned () . filter (| cs | cs . common . suite == CipherSuite :: TLS13_AES_128_GCM_SHA256) . collect () , .. provider . clone () } ; let server_config = ServerConfig :: builder (client_provider . into ()) . finish (kt) ; let server_provider = CryptoProvider { tls13_cipher_suites : provider . tls13_cipher_suites . iter () . cloned () . filter (| cs | cs . common . suite == CipherSuite :: TLS13_AES_256_GCM_SHA384) . collect () , .. provider } ; let client_config = ClientConfig :: builder (server_provider . into ()) . finish (kt) ; (client_config , server_config) }
};
}
