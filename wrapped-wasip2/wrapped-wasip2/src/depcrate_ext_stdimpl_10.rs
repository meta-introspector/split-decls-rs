// Generated macro for impl_10 (impl)
macro_rules! Depcrate_ext_stdimpl_10 {
() => {
// Module: crate::ext::std
// Provides: {"impl_10"}
// Dependencies: {}
impl io :: Write for crate :: io :: streams :: OutputStream { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let n = loop { match self . check_write () . map (NonZeroU64 :: new) { Ok (Some (n)) => { break n ; } Ok (None) => { self . subscribe () . block () ; } Err (StreamError :: Closed) => return Ok (0) , Err (StreamError :: LastOperationFailed (e)) => { return Err (io :: Error :: new (io :: ErrorKind :: Other , e . to_debug_string ())) } } ; } ; let n = n . get () . try_into () . map_err (| e | io :: Error :: new (io :: ErrorKind :: Other , e)) ? ; let n = buf . len () . min (n) ; crate :: io :: streams :: OutputStream :: write (self , & buf [.. n]) . map_err (| e | match e { StreamError :: Closed => io :: ErrorKind :: UnexpectedEof . into () , StreamError :: LastOperationFailed (e) => { io :: Error :: new (io :: ErrorKind :: Other , e . to_debug_string ()) } }) ? ; Ok (n) } fn flush (& mut self) -> io :: Result < () > { self . blocking_flush () . map_err (| e | io :: Error :: new (io :: ErrorKind :: Other , e)) } }
};
}
