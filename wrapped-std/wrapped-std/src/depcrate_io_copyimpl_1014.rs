// Generated macro for impl_1014 (impl)
macro_rules! Depcrate_io_copyimpl_1014 {
() => {
// Module: crate::io::copy
// Provides: {"impl_1014"}
// Dependencies: {}
impl < A : Allocator > BufferedReaderSpec for VecDeque < u8 , A > { fn buffer_size (& self) -> usize { usize :: MAX } fn copy_to (& mut self , to : & mut (impl Write + ? Sized)) -> Result < u64 > { let len = self . len () ; let (front , back) = self . as_slices () ; let bufs = & mut [IoSlice :: new (front) , IoSlice :: new (back)] ; to . write_all_vectored (bufs) ? ; self . clear () ; Ok (len as u64) } }
};
}
