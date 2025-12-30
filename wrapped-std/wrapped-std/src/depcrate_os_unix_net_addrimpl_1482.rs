// Generated macro for impl_1482 (impl)
macro_rules! Depcrate_os_unix_net_addrimpl_1482 {
() => {
// Module: crate::os::unix::net::addr
// Provides: {"impl_1482"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl fmt :: Debug for SocketAddr { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . address () { AddressKind :: Unnamed => write ! (fmt , "(unnamed)") , AddressKind :: Abstract (name) => write ! (fmt , "{name:?} (abstract)") , AddressKind :: Pathname (path) => write ! (fmt , "{path:?} (pathname)") , } } }
};
}
