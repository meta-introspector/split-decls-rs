// Generated macro for impl_1081 (impl)
macro_rules! Depcrate_io_implsimpl_1081 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1081"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < R : Read + ? Sized > Read for & mut R { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { (* * self) . read (buf) } # [inline] fn read_buf (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { (* * self) . read_buf (cursor) } # [inline] fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { (* * self) . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { (* * self) . is_read_vectored () } # [inline] fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { (* * self) . read_to_end (buf) } # [inline] fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { (* * self) . read_to_string (buf) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { (* * self) . read_exact (buf) } # [inline] fn read_buf_exact (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { (* * self) . read_buf_exact (cursor) } }
};
}
