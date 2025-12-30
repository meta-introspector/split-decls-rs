// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_io_cursorimpl_1046 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1046"}
// Dependencies: {}
# [stable (feature = "cursor_box_slice" , since = "1.5.0")] impl < A > Write for Cursor < Box < [u8] , A > > where A : Allocator , { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { slice_write (& mut self . pos , & mut self . inner , buf) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { slice_write_vectored (& mut self . pos , & mut self . inner , bufs) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { slice_write_all (& mut self . pos , & mut self . inner , buf) } # [inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { slice_write_all_vectored (& mut self . pos , & mut self . inner , bufs) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
