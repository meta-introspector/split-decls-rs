// Generated macro for impl_258 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_258 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_258"}
// Dependencies: {}
impl PresharedKeyOffer { # [doc = " Make a new one with one entry."] pub (crate) fn new (id : PresharedKeyIdentity , binder : Vec < u8 >) -> Self { Self { identities : vec ! [id] , binders : vec ! [PresharedKeyBinder :: from (binder)] , } } }
};
}
