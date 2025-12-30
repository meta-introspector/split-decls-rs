// Generated macro for impl_84 (impl)
macro_rules! Depcrate_sockrefimpl_84 {
() => {
// Module: crate::sockref
// Provides: {"impl_84"}
// Dependencies: {}
impl fmt :: Debug for SockRef < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SockRef") . field ("raw" , & self . socket . as_raw ()) . field ("local_addr" , & self . socket . local_addr () . ok ()) . field ("peer_addr" , & self . socket . peer_addr () . ok ()) . finish () } }
};
}
