// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_trusted_for_tls () { let mut metadata = CertificateMetadata { common_name_or_certificate_name : "Test" . to_string () , certificate_serial_number : "1" . to_string () , sha256_fingerprint : "1" . to_string () , trust_bits : "Websites" . to_string () , distrust_for_tls_after_date : "" . to_string () , mozilla_applied_constraints : "" . to_string () , pem_info : "" . to_string () , } ; assert ! (metadata . trusted_for_tls ()) ; metadata . trust_bits = "Email" . to_string () ; assert ! (! metadata . trusted_for_tls ()) ; metadata . trust_bits = "Websites;Email" . to_string () ; assert ! (metadata . trusted_for_tls ()) ; metadata . trust_bits = "Websites" . to_string () ; metadata . distrust_for_tls_after_date = "2000.01.01" . to_string () ; assert ! (! metadata . trusted_for_tls ()) ; let now = Utc :: now () . naive_utc () ; let future_distrust = now . add (Duration :: days (365 * 5)) ; metadata . distrust_for_tls_after_date = future_distrust . format ("%Y.%m.%d") . to_string () ; assert ! (metadata . trusted_for_tls ()) ; let past_distrust = now . add (Duration :: days (- 397)) ; metadata . distrust_for_tls_after_date = past_distrust . format ("%Y.%m.%d") . to_string () ; assert ! (metadata . trusted_for_tls ()) ; let past_distrust = now . add (Duration :: days (- 398)) ; metadata . distrust_for_tls_after_date = past_distrust . format ("%Y.%m.%d") . to_string () ; assert ! (! metadata . trusted_for_tls ()) ; metadata . sha256_fingerprint = EXCLUDED_FINGERPRINTS [0] . to_string () ; assert ! (! metadata . trusted_for_tls ()) ; } }
};
}
