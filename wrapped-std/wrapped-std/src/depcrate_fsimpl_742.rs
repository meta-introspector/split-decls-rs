// Generated macro for impl_742 (impl)
macro_rules! Depcrate_fsimpl_742 {
() => {
// Module: crate::fs
// Provides: {"impl_742"}
// Dependencies: {}
# [stable (feature = "io_traits_arc" , since = "1.73.0")] impl Read for Arc < File > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { (& * * self) . read (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { (& * * self) . read_vectored (bufs) } fn read_buf (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { (& * * self) . read_buf (cursor) } # [inline] fn is_read_vectored (& self) -> bool { (& * * self) . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { (& * * self) . read_to_end (buf) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { (& * * self) . read_to_string (buf) } }
};
}
