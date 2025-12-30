// Generated macro for impl_65 (impl)
macro_rules! Depcrate_channelimpl_65 {
() => {
// Module: crate::channel
// Provides: {"impl_65"}
// Dependencies: {}
impl Write for Stream { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { let locked = self . lock () ; unsafe { let rc = raw :: libssh2_channel_write_ex (locked . raw , locked . id as c_int , data . as_ptr () as * mut _ , data . len () as size_t ,) ; locked . sess . rc (rc as c_int) . map (| () | rc as usize) } . map_err (Into :: into) } fn flush (& mut self) -> io :: Result < () > { let locked = self . lock () ; unsafe { let rc = raw :: libssh2_channel_flush_ex (locked . raw , locked . id as c_int) ; locked . sess . rc (rc) } . map_err (Into :: into) } }
};
}
