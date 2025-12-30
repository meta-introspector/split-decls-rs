// Generated macro for impl_1198 (impl)
macro_rules! Depcrate_io_utilimpl_1198 {
() => {
// Module: crate::io::util
// Provides: {"impl_1198"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Read for Empty { # [inline] fn read (& mut self , _buf : & mut [u8]) -> io :: Result < usize > { Ok (0) } # [inline] fn read_buf (& mut self , _cursor : BorrowedCursor < '_ >) -> io :: Result < () > { Ok (()) } # [inline] fn read_vectored (& mut self , _bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { Ok (0) } # [inline] fn is_read_vectored (& self) -> bool { false } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { if ! buf . is_empty () { Err (io :: Error :: READ_EXACT_EOF) } else { Ok (()) } } # [inline] fn read_buf_exact (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { if cursor . capacity () != 0 { Err (io :: Error :: READ_EXACT_EOF) } else { Ok (()) } } # [inline] fn read_to_end (& mut self , _buf : & mut Vec < u8 >) -> io :: Result < usize > { Ok (0) } # [inline] fn read_to_string (& mut self , _buf : & mut String) -> io :: Result < usize > { Ok (0) } }
};
}
