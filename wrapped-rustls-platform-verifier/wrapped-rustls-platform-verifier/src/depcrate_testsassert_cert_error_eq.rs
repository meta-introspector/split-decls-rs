// Generated macro for assert_cert_error_eq (function)
macro_rules! Depcrate_testsassert_cert_error_eq {
() => {
// Module: crate::tests
// Provides: {"assert_cert_error_eq"}
// Dependencies: {}
pub fn assert_cert_error_eq < E : StdError + PartialEq + 'static > (result : & Result < () , TlsError > , expected : & Result < () , TlsError > , expected_err : Option < & E > ,) { if let Err (InvalidCertificate (CertificateError :: Other (err))) = & expected { let expected_err = expected_err . expect ("error not provided for `Other` case handling") ; let err : & E = err . 0 . downcast_ref () . expect ("incorrect `Other` inner error kind") ; assert_eq ! (err , expected_err) ; } else { assert_eq ! (result , expected) ; } }
};
}
