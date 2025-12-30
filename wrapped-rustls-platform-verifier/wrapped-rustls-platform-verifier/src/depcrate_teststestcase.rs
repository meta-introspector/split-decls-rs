// Generated macro for TestCase (struct)
macro_rules! Depcrate_testsTestCase {
() => {
// Module: crate::tests
// Provides: {"TestCase"}
// Dependencies: {}
struct TestCase < 'a , E : StdError > { # [doc = " The name of the server we're connecting to."] pub reference_id : & 'a str , # [doc = " The certificates presented by the TLS server, in the same order."] pub chain : & 'a [& 'a [u8]] , # [doc = " The stapled OCSP response given to us by Rustls, if any."] pub stapled_ocsp : Option < & 'a [u8] > , # [doc = " The time to use as the current time for verification."] pub verification_time : pki_types :: UnixTime , pub expected_result : Result < () , TlsError > , # [doc = " An error that should be present inside an expected `CertificateError::Other` variant."] # [doc = ""] # [doc = " Set this if the error being tested uses `CertificateError::Other` and not statically known"] # [doc = " variants in [TlsError]"] # [allow (dead_code)] pub other_error : Option < E > , }
};
}
