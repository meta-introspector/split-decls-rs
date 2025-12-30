// Generated macro for impl_192 (impl)
macro_rules! Depcrate_tls_streamimpl_192 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_192"}
// Dependencies: {}
impl < S > Read for TlsStream < S > where S : Read + Write , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let nread = { let read_buf = self . fill_buf () ? ; let nread = cmp :: min (buf . len () , read_buf . len ()) ; buf [.. nread] . copy_from_slice (& read_buf [.. nread]) ; nread } ; self . consume (nread) ; Ok (nread) } }
};
}
