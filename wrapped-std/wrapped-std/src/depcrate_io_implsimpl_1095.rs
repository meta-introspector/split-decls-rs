// Generated macro for impl_1095 (impl)
macro_rules! Depcrate_io_implsimpl_1095 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1095"}
// Dependencies: {}
# [doc = " Write is implemented for `VecDeque<u8>` by appending to the `VecDeque`, growing it as needed."] # [stable (feature = "vecdeque_read_write" , since = "1.63.0")] impl < A : Allocator > Write for VecDeque < u8 , A > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . extend (buf) ; Ok (buf . len ()) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let len = bufs . iter () . map (| b | b . len ()) . sum () ; self . reserve (len) ; for buf in bufs { self . extend (& * * buf) ; } Ok (len) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . extend (buf) ; Ok (()) } # [inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { self . write_vectored (bufs) ? ; Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
