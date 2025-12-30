// Generated macro for impl_1092 (impl)
macro_rules! Depcrate_io_implsimpl_1092 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1092"}
// Dependencies: {}
# [doc = " Write is implemented for `Vec<u8>` by appending to the vector."] # [doc = " The vector will grow as needed."] # [stable (feature = "rust1" , since = "1.0.0")] impl < A : Allocator > Write for Vec < u8 , A > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . extend_from_slice (buf) ; Ok (buf . len ()) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let len = bufs . iter () . map (| b | b . len ()) . sum () ; self . reserve (len) ; for buf in bufs { self . extend_from_slice (buf) ; } Ok (len) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . extend_from_slice (buf) ; Ok (()) } # [inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { self . write_vectored (bufs) ? ; Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
