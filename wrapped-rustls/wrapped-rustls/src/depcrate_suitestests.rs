// Generated macro for tests (module)
macro_rules! Depcrate_suitestests {
() => {
// Module: crate::suites
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: println ; use super :: SupportedCipherSuite ; use crate :: TEST_PROVIDERS ; use crate :: crypto :: tls13_suite ; use crate :: enums :: CipherSuite ; # [test] fn test_scs_is_debug () { for & provider in TEST_PROVIDERS { let aes_128_gcm = tls13_suite (CipherSuite :: TLS13_AES_128_GCM_SHA256 , provider) ; println ! ("{:?}" , SupportedCipherSuite :: Tls13 (aes_128_gcm)) ; } } }
};
}
