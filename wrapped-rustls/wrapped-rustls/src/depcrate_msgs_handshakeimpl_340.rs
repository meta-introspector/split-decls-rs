// Generated macro for impl_340 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_340 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_340"}
// Dependencies: {}
impl Codec < '_ > for ServerEcdhParams { fn encode (& self , bytes : & mut Vec < u8 >) { self . curve_params . encode (bytes) ; self . public . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let cp = EcParameters :: read (r) ? ; let pb = PayloadU8 :: read (r) ? ; Ok (Self { curve_params : cp , public : pb , }) } }
};
}
