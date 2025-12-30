// Generated macro for impl_658 (impl)
macro_rules! Depcrate_secure_transportimpl_658 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_658"}
// Dependencies: {}
impl < S : fmt :: Debug > fmt :: Debug for SslStream < S > { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("SslStream") . field ("context" , & self . ctx) . field ("stream" , self . get_ref ()) . finish () } }
};
}
