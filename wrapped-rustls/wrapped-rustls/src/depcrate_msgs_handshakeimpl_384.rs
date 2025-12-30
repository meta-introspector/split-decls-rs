// Generated macro for impl_384 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_384 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_384"}
// Dependencies: {}
impl Codec < '_ > for HpkeSymmetricCipherSuite { fn encode (& self , bytes : & mut Vec < u8 >) { self . kdf_id . encode (bytes) ; self . aead_id . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { kdf_id : HpkeKdf :: read (r) ? , aead_id : HpkeAead :: read (r) ? , }) } }
};
}
