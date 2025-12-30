// Generated macro for impl_18 (impl)
macro_rules! Depcrate_sockaddrimpl_18 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Debug for SockAddr { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let mut builder = fmt . debug_struct ("SockAddr") ; builder . field ("family" , & self . family ()) ; if let Some (addr) = self . as_inet () { builder . field ("inet" , & addr) ; } else if let Some (addr) = self . as_inet6 () { builder . field ("inet6" , & addr) ; } builder . finish () } }
};
}
