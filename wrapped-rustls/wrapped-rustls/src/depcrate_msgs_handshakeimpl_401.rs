// Generated macro for impl_401 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_401 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_401"}
// Dependencies: {}
impl Codec < '_ > for EncryptedClientHelloOuter { fn encode (& self , bytes : & mut Vec < u8 >) { self . cipher_suite . encode (bytes) ; self . config_id . encode (bytes) ; self . enc . encode (bytes) ; self . payload . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { cipher_suite : HpkeSymmetricCipherSuite :: read (r) ? , config_id : u8 :: read (r) ? , enc : PayloadU16 :: read (r) ? , payload : PayloadU16 :: read (r) ? , }) } }
};
}
