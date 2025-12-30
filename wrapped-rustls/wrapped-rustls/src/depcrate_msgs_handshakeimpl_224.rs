// Generated macro for impl_224 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_224 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_224"}
// Dependencies: {}
impl SessionId { pub (crate) fn random (secure_random : & dyn SecureRandom) -> Result < Self , rand :: GetRandomFailed > { let mut data = [0u8 ; 32] ; secure_random . fill (& mut data) ? ; Ok (Self { data , len : 32 }) } pub (crate) fn empty () -> Self { Self { data : [0u8 ; 32] , len : 0 , } } pub (crate) fn is_empty (& self) -> bool { self . len == 0 } }
};
}
