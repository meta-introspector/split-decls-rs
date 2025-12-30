// Generated macro for impl_227 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_227 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_227"}
// Dependencies: {}
impl UnknownExtension { fn encode (& self , bytes : & mut Vec < u8 >) { self . payload . encode (bytes) ; } fn read (typ : ExtensionType , r : & mut Reader < '_ >) -> Self { let payload = Payload :: read (r) . into_owned () ; Self { typ , payload } } }
};
}
