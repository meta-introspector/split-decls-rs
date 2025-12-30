// Generated macro for load_pem_certs (function)
macro_rules! Depcrateload_pem_certs {
() => {
// Module: crate
// Provides: {"load_pem_certs"}
// Dependencies: {}
fn load_pem_certs (path : & Path , out : & mut CertificateResult) { let iter = match CertificateDer :: pem_file_iter (path) { Ok (iter) => iter , Err (err) => { out . pem_error (err , path) ; return ; } } ; for result in iter { match result { Ok (cert) => out . certs . push (cert) , Err (err) => out . pem_error (err , path) , } } }
};
}
