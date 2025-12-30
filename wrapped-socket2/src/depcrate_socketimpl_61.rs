// Generated macro for impl_61 (impl)
macro_rules! Depcrate_socketimpl_61 {
() => {
// Module: crate::socket
// Provides: {"impl_61"}
// Dependencies: {}
impl Read for Socket { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let buf = unsafe { & mut * (buf as * mut [u8] as * mut [MaybeUninit < u8 >]) } ; self . recv (buf) } # [cfg (not (target_os = "redox"))] fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { let bufs = unsafe { & mut * (bufs as * mut [IoSliceMut < '_ >] as * mut [MaybeUninitSlice < '_ >]) } ; self . recv_vectored (bufs) . map (| (n , _) | n) } }
};
}
