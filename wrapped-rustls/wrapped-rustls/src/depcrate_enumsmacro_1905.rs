// Generated macro for macro_1905 (macro)
macro_rules! Depcrate_enumsmacro_1905 {
() => {
// Module: crate::enums
// Provides: {"macro_1905"}
// Dependencies: {}
enum_builder ! { # [doc = " The `AlertDescription` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u8)] pub enum AlertDescription { CloseNotify => 0x00 , UnexpectedMessage => 0x0a , BadRecordMac => 0x14 , DecryptionFailed => 0x15 , RecordOverflow => 0x16 , DecompressionFailure => 0x1e , HandshakeFailure => 0x28 , NoCertificate => 0x29 , BadCertificate => 0x2a , UnsupportedCertificate => 0x2b , CertificateRevoked => 0x2c , CertificateExpired => 0x2d , CertificateUnknown => 0x2e , IllegalParameter => 0x2f , UnknownCa => 0x30 , AccessDenied => 0x31 , DecodeError => 0x32 , DecryptError => 0x33 , ExportRestriction => 0x3c , ProtocolVersion => 0x46 , InsufficientSecurity => 0x47 , InternalError => 0x50 , InappropriateFallback => 0x56 , UserCanceled => 0x5a , NoRenegotiation => 0x64 , MissingExtension => 0x6d , UnsupportedExtension => 0x6e , CertificateUnobtainable => 0x6f , UnrecognizedName => 0x70 , BadCertificateStatusResponse => 0x71 , BadCertificateHashValue => 0x72 , UnknownPskIdentity => 0x73 , CertificateRequired => 0x74 , NoApplicationProtocol => 0x78 , EncryptedClientHelloRequired => 0x79 , } }
};
}
