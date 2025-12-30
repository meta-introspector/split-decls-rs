// Generated macro for impl_1137 (impl)
macro_rules! Depcrate_io_stdioimpl_1137 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1137"}
// Dependencies: {}
impl Read for StdinRaw { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { handle_ebadf (self . 0 . read (buf) , | | Ok (0)) } fn read_buf (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { handle_ebadf (self . 0 . read_buf (buf) , | | Ok (())) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { handle_ebadf (self . 0 . read_vectored (bufs) , | | Ok (0)) } # [inline] fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { if buf . is_empty () { return Ok (()) ; } handle_ebadf (self . 0 . read_exact (buf) , | | Err (io :: Error :: READ_EXACT_EOF)) } fn read_buf_exact (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { if buf . capacity () == 0 { return Ok (()) ; } handle_ebadf (self . 0 . read_buf_exact (buf) , | | Err (io :: Error :: READ_EXACT_EOF)) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { handle_ebadf (self . 0 . read_to_end (buf) , | | Ok (0)) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { handle_ebadf (self . 0 . read_to_string (buf) , | | Ok (0)) } }
};
}
