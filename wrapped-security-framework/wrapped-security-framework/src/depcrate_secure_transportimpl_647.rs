// Generated macro for impl_647 (impl)
macro_rules! Depcrate_secure_transportimpl_647 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_647"}
// Dependencies: {}
impl fmt :: Debug for SslContext { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("SslContext") ; if let Ok (state) = self . state () { builder . field ("state" , & state) ; } builder . finish () } }
};
}
