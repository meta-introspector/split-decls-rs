// Generated macro for impl_1044 (impl)
macro_rules! Depcrate_io_cursorimpl_1044 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1044"}
// Dependencies: {}
# [stable (feature = "cursor_mut_vec" , since = "1.25.0")] impl < A > Write for Cursor < & mut Vec < u8 , A > > where A : Allocator , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { vec_write_all (& mut self . pos , self . inner , buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { vec_write_all_vectored (& mut self . pos , self . inner , bufs) } # [inline] fn is_write_vectored (& self) -> bool { true } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { vec_write_all (& mut self . pos , self . inner , buf) ? ; Ok (()) } fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { vec_write_all_vectored (& mut self . pos , self . inner , bufs) ? ; Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
