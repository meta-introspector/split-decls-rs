// Generated macro for SignedData (struct)
macro_rules! Depcrate_signed_dataSignedData {
() => {
// Module: crate::signed_data
// Provides: {"SignedData"}
// Dependencies: {}
# [doc = " X.509 certificates and related items that are signed are almost always"] # [doc = " encoded in the format \"tbs||signatureAlgorithm||signature\". This structure"] # [doc = " captures this pattern."] # [derive (Debug)] pub (crate) struct SignedData < 'a > { # [doc = " The signed data. This would be `tbsCertificate` in the case of an X.509"] # [doc = " certificate, `tbsResponseData` in the case of an OCSP response, `tbsCertList`"] # [doc = " in the case of a CRL, and the data nested in the `digitally-signed` construct for"] # [doc = " TLS 1.2 signed data."] pub (crate) data : untrusted :: Input < 'a > , # [doc = " The value of the `AlgorithmIdentifier`. This would be"] # [doc = " `signatureAlgorithm` in the case of an X.509 certificate, OCSP"] # [doc = " response or CRL. This would have to be synthesized in the case of TLS 1.2"] # [doc = " signed data, since TLS does not identify algorithms by ASN.1 OIDs."] pub (crate) algorithm : untrusted :: Input < 'a > , # [doc = " The value of the signature. This would be `signature` in an X.509"] # [doc = " certificate, OCSP response or CRL. This would be the value of"] # [doc = " `DigitallySigned.signature` for TLS 1.2 signed data."] pub (crate) signature : untrusted :: Input < 'a > , }
};
}
