// Generated macro for macro_157 (macro)
macro_rules! Depcrate_msgs_enumsmacro_157 {
() => {
// Module: crate::msgs::enums
// Provides: {"macro_157"}
// Dependencies: {}
enum_builder ! { # [doc = " The `ExtensionType` TLS protocol enum.  Values in this enum are taken"] # [doc = " from the various RFCs covering TLS, and are listed by IANA."] # [doc = " The `Unknown` item is used when processing unrecognized ordinals."] # [repr (u16)] pub enum ExtensionType { ServerName => 0x0000 , MaxFragmentLength => 0x0001 , ClientCertificateUrl => 0x0002 , TrustedCAKeys => 0x0003 , TruncatedHMAC => 0x0004 , StatusRequest => 0x0005 , UserMapping => 0x0006 , ClientAuthz => 0x0007 , ServerAuthz => 0x0008 , CertificateType => 0x0009 , EllipticCurves => 0x000a , ECPointFormats => 0x000b , SRP => 0x000c , SignatureAlgorithms => 0x000d , UseSRTP => 0x000e , Heartbeat => 0x000f , ALProtocolNegotiation => 0x0010 , SCT => 0x0012 , ClientCertificateType => 0x0013 , ServerCertificateType => 0x0014 , Padding => 0x0015 , ExtendedMasterSecret => 0x0017 , CompressCertificate => 0x001b , SessionTicket => 0x0023 , PreSharedKey => 0x0029 , EarlyData => 0x002a , SupportedVersions => 0x002b , Cookie => 0x002c , PSKKeyExchangeModes => 0x002d , TicketEarlyDataInfo => 0x002e , CertificateAuthorities => 0x002f , OIDFilters => 0x0030 , PostHandshakeAuth => 0x0031 , SignatureAlgorithmsCert => 0x0032 , KeyShare => 0x0033 , TransportParameters => 0x0039 , NextProtocolNegotiation => 0x3374 , ChannelId => 0x754f , RenegotiationInfo => 0xff01 , EncryptedClientHello => 0xfe0d , EncryptedClientHelloOuterExtensions => 0xfd00 , } }
};
}
