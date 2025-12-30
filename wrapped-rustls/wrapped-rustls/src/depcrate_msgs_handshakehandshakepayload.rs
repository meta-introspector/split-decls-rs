// Generated macro for HandshakePayload (enum)
macro_rules! Depcrate_msgs_handshakeHandshakePayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"HandshakePayload"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum HandshakePayload < 'a > { HelloRequest , ClientHello (ClientHelloPayload) , ServerHello (ServerHelloPayload) , HelloRetryRequest (HelloRetryRequest) , Certificate (CertificateChain < 'a >) , CertificateTls13 (CertificatePayloadTls13 < 'a >) , CompressedCertificate (CompressedCertificatePayload < 'a >) , ServerKeyExchange (ServerKeyExchangePayload) , CertificateRequest (CertificateRequestPayload) , CertificateRequestTls13 (CertificateRequestPayloadTls13) , CertificateVerify (DigitallySignedStruct) , ServerHelloDone , EndOfEarlyData , ClientKeyExchange (Payload < 'a >) , NewSessionTicket (NewSessionTicketPayload) , NewSessionTicketTls13 (NewSessionTicketPayloadTls13) , EncryptedExtensions (Box < ServerExtensions < 'a > >) , KeyUpdate (KeyUpdateRequest) , Finished (Payload < 'a >) , CertificateStatus (CertificateStatus < 'a >) , MessageHash (Payload < 'a >) , Unknown ((HandshakeType , Payload < 'a >)) , }
};
}
