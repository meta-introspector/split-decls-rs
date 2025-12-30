// Generated macro for impl_500 (impl)
macro_rules! Depcrate_io_errnoimpl_500 {
() => {
// Module: crate::io::errno
// Provides: {"impl_500"}
// Dependencies: {}
impl fmt :: Display for Errno { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "std")] { std :: io :: Error :: from (* self) . fmt (f) } # [cfg (not (feature = "std"))] { write ! (f , "os error {}" , self . raw_os_error ()) } } }
};
}
