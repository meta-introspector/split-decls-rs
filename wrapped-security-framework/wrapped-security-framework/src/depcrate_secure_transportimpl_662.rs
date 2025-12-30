// Generated macro for impl_662 (impl)
macro_rules! Depcrate_secure_transportimpl_662 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_662"}
// Dependencies: {}
impl < S : Read + Write > Write for SslStream < S > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if buf . is_empty () { return Ok (0) ; } unsafe { let mut nwritten = 0 ; let ret = SSLWrite (self . ctx . 0 , buf . as_ptr () . cast () , buf . len () , & mut nwritten ,) ; if nwritten > 0 { Ok (nwritten) } else { Err (self . get_error (ret)) } } } fn flush (& mut self) -> io :: Result < () > { self . connection_mut () . stream . flush () } }
};
}
