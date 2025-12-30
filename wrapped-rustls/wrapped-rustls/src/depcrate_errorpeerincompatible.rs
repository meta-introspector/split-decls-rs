// Generated macro for PeerIncompatible (enum)
macro_rules! Depcrate_errorPeerIncompatible {
() => {
// Module: crate::error
// Provides: {"PeerIncompatible"}
// Dependencies: {}
# [doc = " The set of cases where we failed to make a connection because a peer"] # [doc = " doesn't support a TLS version/feature we require."] # [doc = ""] # [doc = " This is `non_exhaustive`: we might add or stop using items here in minor"] # [doc = " versions."] # [expect (missing_docs)] # [non_exhaustive] # [derive (Debug , PartialEq , Clone)] pub enum PeerIncompatible { EcPointsExtensionRequired , ExtendedMasterSecretExtensionRequired , IncorrectCertificateTypeExtension , KeyShareExtensionRequired , MultipleRawKeys , NamedGroupsExtensionRequired , NoCertificateRequestSignatureSchemesInCommon , NoCipherSuitesInCommon , NoEcPointFormatsInCommon , NoKxGroupsInCommon , NoSignatureSchemesInCommon , NoServerNameProvided , NullCompressionRequired , ServerDoesNotSupportTls12Or13 , ServerSentHelloRetryRequestWithUnknownExtension , ServerTlsVersionIsDisabledByOurConfig , SignatureAlgorithmsExtensionRequired , SupportedVersionsExtensionRequired , Tls12NotOffered , Tls12NotOfferedOrEnabled , Tls13RequiredForQuic , UncompressedEcPointsRequired , UnknownCertificateType (u8) , UnsolicitedCertificateTypeExtension , }
};
}
