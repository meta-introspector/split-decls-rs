// Generated macro for impl_1680 (impl)
macro_rules! Depcrate_systemimpl_1680 {
() => {
// Module: crate::system
// Provides: {"impl_1680"}
// Dependencies: {}
impl fmt :: Debug for Uname { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (not (linux_kernel))] { write ! (f , "{:?} {:?} {:?} {:?} {:?}" , self . sysname () , self . nodename () , self . release () , self . version () , self . machine () ,) } # [cfg (linux_kernel)] { write ! (f , "{:?} {:?} {:?} {:?} {:?} {:?}" , self . sysname () , self . nodename () , self . release () , self . version () , self . machine () , self . domainname () ,) } } }
};
}
