// Generated macro for impl_1149 (impl)
macro_rules! Depcrate_io_stdioimpl_1149 {
() => {
// Module: crate::io::stdio
// Provides: {"impl_1149"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Read for StdinLock < '_ > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } fn read_buf (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . inner . read_buf (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . inner . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { self . inner . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . inner . read_to_end (buf) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { self . inner . read_to_string (buf) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . inner . read_exact (buf) } fn read_buf_exact (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { self . inner . read_buf_exact (cursor) } }
};
}
