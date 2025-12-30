// Generated macro for test (module)
macro_rules! Depcrate_trust_settingstest {
() => {
// Module: crate::trust_settings
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: test :: certificate ; fn list_for_domain (domain : Domain) { println ! ("--- domain: {domain:?}") ; let ts = TrustSettings :: new (domain) ; let iterator = ts . iter () . unwrap () ; for (i , cert) in iterator . enumerate () { println ! ("cert({i:?}) = {cert:?}") ; println ! ("  settings = {:?}" , ts . tls_trust_settings_for_certificate (& cert)) ; } println ! ("---") ; } # [test] fn list_for_user () { list_for_domain (Domain :: User) ; } # [test] fn list_for_system () { list_for_domain (Domain :: System) ; } # [test] fn list_for_admin () { list_for_domain (Domain :: Admin) ; } # [test] fn test_system_certs_are_present () { let system = TrustSettings :: new (Domain :: System) . iter () . unwrap () . count () ; assert ! (system > 100) ; } # [test] fn test_isrg_root_exists_and_is_trusted () { let ts = TrustSettings :: new (Domain :: System) ; assert_eq ! (ts . iter () . unwrap () . find (| cert | cert . subject_summary () == "ISRG Root X1") . and_then (| cert | ts . tls_trust_settings_for_certificate (& cert) . unwrap ()) , None) ; } # [test] fn test_unknown_cert_is_not_trusted () { let ts = TrustSettings :: new (Domain :: System) ; let cert = certificate () ; assert_eq ! (ts . tls_trust_settings_for_certificate (& cert) . err () . unwrap () . message () , Some ("The specified item could not be found in the keychain." . into ())) ; } }
};
}
