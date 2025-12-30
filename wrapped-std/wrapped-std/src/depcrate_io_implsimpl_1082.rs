// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_io_implsimpl_1082 {
() => {
// Module: crate::io::impls
// Provides: {"impl_1082"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < W : Write + ? Sized > Write for & mut W { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (* * self) . write (buf) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { (* * self) . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { (* * self) . is_write_vectored () } # [inline] fn flush (& mut self) -> io :: Result < () > { (* * self) . flush () } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { (* * self) . write_all (buf) } # [inline] fn write_all_vectored (& mut self , bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { (* * self) . write_all_vectored (bufs) } # [inline] fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { (* * self) . write_fmt (fmt) } }
};
}
