// Generated macro for impl_218 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_218 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_218"}
// Dependencies: {}
impl Random { pub (crate) fn new (secure_random : & dyn SecureRandom) -> Result < Self , rand :: GetRandomFailed > { let mut data = [0u8 ; 32] ; secure_random . fill (& mut data) ? ; Ok (Self (data)) } }
};
}
