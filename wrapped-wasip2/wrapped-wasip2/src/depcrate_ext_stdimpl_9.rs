// Generated macro for impl_9 (impl)
macro_rules! Depcrate_ext_stdimpl_9 {
() => {
// Module: crate::ext::std
// Provides: {"impl_9"}
// Dependencies: {}
impl io :: Read for crate :: io :: streams :: InputStream { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let n = buf . len () . try_into () . map_err (| e | io :: Error :: new (io :: ErrorKind :: Other , e)) ? ; match self . blocking_read (n) { Ok (chunk) => { let n = chunk . len () ; if n > buf . len () { return Err (io :: Error :: new (io :: ErrorKind :: Other , "more bytes read than requested" ,)) ; } buf [.. n] . copy_from_slice (& chunk) ; Ok (n) } Err (StreamError :: Closed) => Ok (0) , Err (StreamError :: LastOperationFailed (e)) => { Err (io :: Error :: new (io :: ErrorKind :: Other , e . to_debug_string ())) } } } }
};
}
