// Generated macro for macro_1907 (macro)
macro_rules! Depcrate_enumsmacro_1907 {
() => {
// Module: crate::enums
// Provides: {"macro_1907"}
// Dependencies: {}
enum_builder ! { # [doc = " The `HandshakeType` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u8)] pub enum HandshakeType { HelloRequest => 0x00 , ClientHello => 0x01 , ServerHello => 0x02 , HelloVerifyRequest => 0x03 , NewSessionTicket => 0x04 , EndOfEarlyData => 0x05 , HelloRetryRequest => 0x06 , EncryptedExtensions => 0x08 , Certificate => 0x0b , ServerKeyExchange => 0x0c , CertificateRequest => 0x0d , ServerHelloDone => 0x0e , CertificateVerify => 0x0f , ClientKeyExchange => 0x10 , Finished => 0x14 , CertificateURL => 0x15 , CertificateStatus => 0x16 , KeyUpdate => 0x18 , CompressedCertificate => 0x19 , MessageHash => 0xfe , } }
};
}
