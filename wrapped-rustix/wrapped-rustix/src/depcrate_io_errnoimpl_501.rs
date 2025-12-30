// Generated macro for impl_501 (impl)
macro_rules! Depcrate_io_errnoimpl_501 {
() => {
// Module: crate::io::errno
// Provides: {"impl_501"}
// Dependencies: {}
impl fmt :: Debug for Errno { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "std")] { std :: io :: Error :: from (* self) . fmt (f) } # [cfg (not (feature = "std"))] { write ! (f , "os error {}" , self . raw_os_error ()) } } }
};
}
