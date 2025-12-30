// Generated macro for impl_165 (impl)
macro_rules! Depcrate_io_read_bufimpl_165 {
() => {
// Module: crate::io::read_buf
// Provides: {"impl_165"}
// Dependencies: {}
impl fmt :: Debug for ReadBuf < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReadBuf") . field ("filled" , & self . filled) . field ("initialized" , & self . initialized) . field ("capacity" , & self . capacity ()) . finish () } }
};
}
