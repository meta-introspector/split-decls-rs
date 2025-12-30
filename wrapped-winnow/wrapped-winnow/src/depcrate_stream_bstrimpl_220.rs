// Generated macro for impl_220 (impl)
macro_rules! Depcrate_stream_bstrimpl_220 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_220"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl fmt :: Display for BStr { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { alloc :: string :: String :: from_utf8_lossy (self . as_bytes ()) . fmt (f) } }
};
}
