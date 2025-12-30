// Generated macro for test (module)
macro_rules! Depcrate_certificatetest {
() => {
// Module: crate::certificate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: test :: certificate ; # [cfg (any (feature = "OSX_10_12" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] use x509_parser :: prelude :: * ; # [test] fn subject_summary () { let cert = certificate () ; assert_eq ! ("foobar.com" , cert . subject_summary ()) ; } # [test] fn email_addresses () { let cert = certificate () ; assert_eq ! (Vec ::< String >:: new () , cert . email_addresses () . unwrap ()) ; } # [test] # [cfg (any (feature = "OSX_10_12" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] fn issuer () { let cert = certificate () ; let issuer = cert . issuer () ; let (_ , name) = X509Name :: from_der (& issuer) . unwrap () ; let name_str = name . to_string_with_registry (oid_registry ()) . unwrap () ; assert_eq ! ("C=US, ST=CALIFORNIA, L=PALO ALTO, O=FOOBAR LLC, OU=DEV LAND, CN=FOOBAR.COM" , name_str) ; } # [test] # [cfg (any (feature = "OSX_10_12" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] fn subject () { let cert = certificate () ; let subject = cert . subject () ; let (_ , name) = X509Name :: from_der (& subject) . unwrap () ; let name_str = name . to_string_with_registry (oid_registry ()) . unwrap () ; assert_eq ! ("C=US, ST=CALIFORNIA, L=PALO ALTO, O=FOOBAR LLC, OU=DEV LAND, CN=FOOBAR.COM" , name_str) ; } }
};
}
