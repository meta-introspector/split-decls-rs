// Generated macro for tests (module)
macro_rules! Depcrate_tls13tests {
() => {
// Module: crate::tls13
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: TEST_PROVIDERS ; use crate :: crypto :: tls13_suite ; use crate :: enums :: CipherSuite ; # [test] fn test_can_resume_to () { for & provider in TEST_PROVIDERS { let Some (cha_poly) = provider . tls13_cipher_suites . iter () . find (| cs | cs . common . suite == CipherSuite :: TLS13_CHACHA20_POLY1305_SHA256) else { continue ; } ; let aes_128_gcm = tls13_suite (CipherSuite :: TLS13_AES_128_GCM_SHA256 , provider) ; assert ! (aes_128_gcm . can_resume_from (cha_poly) . is_some ()) ; let aes_256_gcm = tls13_suite (CipherSuite :: TLS13_AES_256_GCM_SHA384 , provider) ; assert ! (aes_256_gcm . can_resume_from (cha_poly) . is_none ()) ; } } }
};
}
