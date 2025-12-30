// Generated macro for load_private_key (function)
macro_rules! Depcrateload_private_key {
() => {
// Module: crate
// Provides: {"load_private_key"}
// Dependencies: {}
fn load_private_key () -> io :: Result < PrivateKeyDer < 'static > > { static KEY : & [u8] = include_bytes ! ("./sample.rsa") ; let buf = io :: Cursor :: new (KEY) ; let mut reader = io :: BufReader :: new (buf) ; rustls_pemfile :: private_key (& mut reader) . map (| key | key . unwrap ()) }
};
}
