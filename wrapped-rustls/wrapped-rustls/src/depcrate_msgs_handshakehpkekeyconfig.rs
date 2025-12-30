// Generated macro for HpkeKeyConfig (struct)
macro_rules! Depcrate_msgs_handshakeHpkeKeyConfig {
() => {
// Module: crate::msgs::handshake
// Provides: {"HpkeKeyConfig"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub (crate) struct HpkeKeyConfig { pub config_id : u8 , pub kem_id : HpkeKem , # [doc = " draft-ietf-tls-esni-24: `opaque HpkePublicKey<1..2^16-1>;`"] pub public_key : PayloadU16 < NonEmpty > , pub symmetric_cipher_suites : Vec < HpkeSymmetricCipherSuite > , }
};
}
