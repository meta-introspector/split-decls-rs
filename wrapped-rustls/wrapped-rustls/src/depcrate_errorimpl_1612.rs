// Generated macro for impl_1612 (impl)
macro_rules! Depcrate_errorimpl_1612 {
() => {
// Module: crate::error
// Provides: {"impl_1612"}
// Dependencies: {}
impl From < CertificateError > for AlertDescription { fn from (e : CertificateError) -> Self { use CertificateError :: * ; match e { BadEncoding | UnhandledCriticalExtension | NotValidForName | NotValidForNameContext { .. } => Self :: BadCertificate , Expired | ExpiredContext { .. } | NotValidYet | NotValidYetContext { .. } => { Self :: CertificateExpired } Revoked => Self :: CertificateRevoked , UnknownIssuer | UnknownRevocationStatus | ExpiredRevocationList | ExpiredRevocationListContext { .. } => Self :: UnknownCa , InvalidOcspResponse => Self :: BadCertificateStatusResponse , BadSignature | UnsupportedSignatureAlgorithm { .. } | UnsupportedSignatureAlgorithmForPublicKey { .. } => Self :: DecryptError , InvalidPurpose | InvalidPurposeContext { .. } => Self :: UnsupportedCertificate , ApplicationVerificationFailure => Self :: AccessDenied , Other (..) => Self :: CertificateUnknown , } } }
};
}
