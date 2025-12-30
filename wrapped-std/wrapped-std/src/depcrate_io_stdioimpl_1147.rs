// Generated macro for impl_1147 (impl)
macro_rules! Depcrate_io_stdioimpl_1147 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1147"}
// Dependencies: {}
# [stable (feature = "read_shared_stdin" , since = "1.78.0")] impl Read for & Stdin { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . lock () . read (buf) } fn read_buf (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . lock () . read_buf (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . lock () . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { self . lock () . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . lock () . read_to_end (buf) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { self . lock () . read_to_string (buf) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . lock () . read_exact (buf) } fn read_buf_exact (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { self . lock () . read_buf_exact (cursor) } }
};
}
