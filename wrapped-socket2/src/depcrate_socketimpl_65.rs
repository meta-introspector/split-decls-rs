// Generated macro for impl_65 (impl)
macro_rules! Depcrate_socketimpl_65 {
() => {
// Module: crate::socket
// Provides: {"impl_65"}
// Dependencies: {}
impl fmt :: Debug for Socket { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Socket") . field ("raw" , & self . as_raw ()) . field ("local_addr" , & self . local_addr () . ok ()) . field ("peer_addr" , & self . peer_addr () . ok ()) . finish () } }
};
}
