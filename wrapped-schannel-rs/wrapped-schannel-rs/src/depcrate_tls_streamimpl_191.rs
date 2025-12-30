// Generated macro for impl_191 (impl)
macro_rules! Depcrate_tls_streamimpl_191 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_191"}
// Dependencies: {}
impl < S > Write for TlsStream < S > where S : Read + Write , { # [doc = " In the case of a WouldBlock error, we expect another call"] # [doc = " starting with the same input data"] # [doc = " This is similar to the use of ACCEPT_MOVING_WRITE_BUFFER in openssl"] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let sizes = match self . initialize () ? { Some (sizes) => sizes , None => { return Err (io :: Error :: from_raw_os_error (Foundation :: SEC_E_CONTEXT_EXPIRED as i32 ,)) } } ; if self . out_buf . position () == self . out_buf . get_ref () . len () as u64 { let len = cmp :: min (buf . len () , sizes . cbMaximumMessage as usize) ; self . encrypt (& buf [.. len] , & sizes) ? ; self . last_write_len = len ; } self . write_out () ? ; Ok (self . last_write_len) } fn flush (& mut self) -> io :: Result < () > { self . write_out () ? ; self . stream . flush () } }
};
}
