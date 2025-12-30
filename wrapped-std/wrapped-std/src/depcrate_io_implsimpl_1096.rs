// Generated macro for impl_1096 (impl)
macro_rules! Depcrate_io_implsimpl_1096 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1096"}
// Dependencies: {}
# [unstable (feature = "read_buf" , issue = "78485")] impl < 'a > io :: Write for core :: io :: BorrowedCursor < 'a > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let amt = cmp :: min (buf . len () , self . capacity ()) ; self . append (& buf [.. amt]) ; Ok (amt) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let mut nwritten = 0 ; for buf in bufs { let n = self . write (buf) ? ; nwritten += n ; if n < buf . len () { break ; } } Ok (nwritten) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { if self . write (buf) ? < buf . len () { Err (io :: Error :: WRITE_ALL_EOF) } else { Ok (()) } } # [inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { for buf in bufs { if self . write (buf) ? < buf . len () { return Err (io :: Error :: WRITE_ALL_EOF) ; } } Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
