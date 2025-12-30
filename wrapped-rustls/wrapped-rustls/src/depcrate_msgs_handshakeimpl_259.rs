// Generated macro for impl_259 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_259 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_259"}
// Dependencies: {}
impl Codec < '_ > for PresharedKeyOffer { fn encode (& self , bytes : & mut Vec < u8 >) { self . identities . encode (bytes) ; self . binders . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { identities : Vec :: read (r) ? , binders : Vec :: read (r) ? , }) } }
};
}
