// Generated macro for load_certs_from_paths_internal (function)
macro_rules! Depcrateload_certs_from_paths_internal {
() => {
// Module: crate
// Provides: {"load_certs_from_paths_internal"}
// Dependencies: {}
fn load_certs_from_paths_internal (file : Option < & Path > , dir : & [impl AsRef < Path >] ,) -> CertificateResult { let mut out = CertificateResult :: default () ; if file . is_none () && dir . is_empty () { return out ; } if let Some (cert_file) = file { load_pem_certs (cert_file , & mut out) ; } for cert_dir in dir . iter () { load_pem_certs_from_dir (cert_dir . as_ref () , & mut out) ; } out . certs . sort_unstable_by (| a , b | a . cmp (b)) ; out . certs . dedup () ; out }
};
}
