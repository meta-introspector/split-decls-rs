// Generated macro for load_certs (function)
macro_rules! Depcrateload_certs {
() => {
// Module: crate
// Provides: {"load_certs"}
// Dependencies: {}
fn load_certs () -> io :: Result < Vec < CertificateDer < 'static > > > { static CERTS : & [u8] = include_bytes ! ("./sample.pem") ; let buf = io :: Cursor :: new (CERTS) ; let mut reader = io :: BufReader :: new (buf) ; rustls_pemfile :: certs (& mut reader) . collect () }
};
}
