// Generated macro for load_native_certs (function)
macro_rules! Depcrate_windowsload_native_certs {
() => {
// Module: crate::windows
// Provides: {"load_native_certs"}
// Dependencies: {}
pub fn load_native_certs () -> CertificateResult { let mut result = CertificateResult :: default () ; let current_user_store = match CertStore :: open_current_user ("ROOT") { Ok (store) => store , Err (err) => { result . os_error (err . into () , "failed to open current user certificate store") ; return result ; } } ; for cert in current_user_store . certs () { if usable_for_rustls (cert . valid_uses () . unwrap ()) && cert . is_time_valid () . unwrap () { result . certs . push (CertificateDer :: from (cert . to_der () . to_vec ())) ; } } result }
};
}
