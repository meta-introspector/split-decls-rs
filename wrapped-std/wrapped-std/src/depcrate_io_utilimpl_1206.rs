// Generated macro for impl_1206 (impl)
macro_rules! Depcrate_io_utilimpl_1206 {
() => {
// Module: crate::io::util
// Provides: {"impl_1206"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Read for Repeat { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { buf . fill (self . byte) ; Ok (buf . len ()) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { buf . fill (self . byte) ; Ok (()) } # [inline] fn read_buf (& mut self , mut buf : BorrowedCursor < '_ >) -> io :: Result < () > { unsafe { buf . as_mut () } . write_filled (self . byte) ; unsafe { buf . advance_unchecked (buf . capacity ()) } ; Ok (()) } # [inline] fn read_buf_exact (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . read_buf (buf) } # [doc = " This function is not supported by `io::Repeat`, because there's no end of its data"] fn read_to_end (& mut self , _ : & mut Vec < u8 >) -> io :: Result < usize > { Err (io :: Error :: from (io :: ErrorKind :: OutOfMemory)) } # [doc = " This function is not supported by `io::Repeat`, because there's no end of its data"] fn read_to_string (& mut self , _ : & mut String) -> io :: Result < usize > { Err (io :: Error :: from (io :: ErrorKind :: OutOfMemory)) } # [inline] fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { let mut nwritten = 0 ; for buf in bufs { nwritten += self . read (buf) ? ; } Ok (nwritten) } # [inline] fn is_read_vectored (& self) -> bool { true } }
};
}
