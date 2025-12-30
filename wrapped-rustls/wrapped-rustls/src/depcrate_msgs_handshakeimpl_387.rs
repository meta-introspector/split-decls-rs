// Generated macro for impl_387 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_387 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_387"}
// Dependencies: {}
impl Codec < '_ > for HpkeKeyConfig { fn encode (& self , bytes : & mut Vec < u8 >) { self . config_id . encode (bytes) ; self . kem_id . encode (bytes) ; self . public_key . encode (bytes) ; self . symmetric_cipher_suites . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { config_id : u8 :: read (r) ? , kem_id : HpkeKem :: read (r) ? , public_key : PayloadU16 :: read (r) ? , symmetric_cipher_suites : Vec :: < HpkeSymmetricCipherSuite > :: read (r) ? , }) } }
};
}
