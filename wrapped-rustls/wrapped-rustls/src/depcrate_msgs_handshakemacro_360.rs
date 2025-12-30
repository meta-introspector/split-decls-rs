// Generated macro for macro_360 (macro)
macro_rules! Depcrate_msgs_handshakemacro_360 {
() => {
// Module: crate::msgs::handshake
// Provides: {"macro_360"}
// Dependencies: {}
extension_struct ! { pub (crate) struct CertificateRequestExtensions { ExtensionType :: SignatureAlgorithms => pub (crate) signature_algorithms : Option < Vec < SignatureScheme >>, ExtensionType :: CertificateAuthorities => pub (crate) authority_names : Option < Vec < DistinguishedName >>, ExtensionType :: CompressCertificate => pub (crate) certificate_compression_algorithms : Option < Vec < CertificateCompressionAlgorithm >>, } }
};
}
