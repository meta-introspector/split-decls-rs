// Generated macro for tests (module)
macro_rules! Depcrate_end_entitytests {
() => {
// Module: crate::end_entity
// Provides: {"tests"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg (test)] mod tests { use super :: * ; use crate :: test_utils ; use crate :: test_utils :: RCGEN_SIGNATURE_ALG ; use std :: prelude :: v1 :: * ; # [test] fn printable_string_common_name () { const DNS_NAME : & str = "test.example.com" ; let issuer = test_utils :: make_issuer ("Test") ; let ee_cert = { let mut params = test_utils :: end_entity_params (vec ! [DNS_NAME . to_string ()]) ; params . distinguished_name . push (rcgen :: DnType :: CommonName , rcgen :: DnValue :: PrintableString (rcgen :: string :: PrintableString :: try_from ("example.com") . unwrap () ,) ,) ; params . signed_by (& rcgen :: KeyPair :: generate_for (RCGEN_SIGNATURE_ALG) . unwrap () , & issuer ,) . expect ("failed to make ee cert (this is a test bug)") } ; expect_dns_name (ee_cert . der () , DNS_NAME) ; } # [test] fn empty_sequence_common_name () { let ee_cert_der = { let bytes = include_bytes ! ("../tests/misc/empty_sequence_common_name.der") ; CertificateDer :: from (& bytes [..]) } ; expect_dns_name (& ee_cert_der , "example.com") ; } fn expect_dns_name (der : & CertificateDer < '_ > , name : & str) { let cert = EndEntityCert :: try_from (der) . expect ("should parse end entity certificate correctly") ; let mut names = cert . valid_dns_names () ; assert_eq ! (names . next () , Some (name)) ; assert_eq ! (names . next () , None) ; } }
};
}
