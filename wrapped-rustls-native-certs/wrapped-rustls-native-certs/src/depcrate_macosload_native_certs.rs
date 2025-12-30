// Generated macro for load_native_certs (function)
macro_rules! Depcrate_macosload_native_certs {
() => {
// Module: crate::macos
// Provides: {"load_native_certs"}
// Dependencies: {}
pub fn load_native_certs () -> CertificateResult { let mut result = CertificateResult :: default () ; let mut all_certs = HashMap :: new () ; for domain in & [Domain :: User , Domain :: Admin , Domain :: System] { let ts = TrustSettings :: new (* domain) ; let iter = match ts . iter () { Ok (iter) => iter , Err (err) => { result . os_error (err . into () , match domain { Domain :: User => "failed to load user trust settings" , Domain :: Admin => "failed to load admin trust settings" , Domain :: System => "failed to load system trust settings" , } ,) ; continue ; } } ; for cert in iter { let der = cert . to_der () ; let trusted = match ts . tls_trust_settings_for_certificate (& cert) { Ok (trusted) => trusted . unwrap_or (TrustSettingsForCertificate :: TrustRoot) , Err (err) => { result . os_error (err . into () , "certificate not trusted") ; continue ; } } ; all_certs . entry (der) . or_insert (trusted) ; } } for (der , trusted) in all_certs . drain () { use TrustSettingsForCertificate :: * ; if let TrustRoot | TrustAsRoot = trusted { result . certs . push (CertificateDer :: from (der)) ; } } result }
};
}
