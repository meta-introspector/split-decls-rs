// Generated macro for verify_tls13_signature (function)
macro_rules! Depcrate_webpki_verifyverify_tls13_signature {
() => {
// Module: crate::webpki::verify
// Provides: {"verify_tls13_signature"}
// Dependencies: {}
# [doc = " Verify a message signature using the `cert` public key and the first TLS 1.3 compatible"] # [doc = " supported scheme."] # [doc = ""] # [doc = " This function verifies the `dss` signature over `message` using the subject public key from"] # [doc = " `cert`. Unlike [verify_tls12_signature], this function only tries the first matching scheme. See"] # [doc = " [WebPkiSupportedAlgorithms::mapping] for more information."] pub fn verify_tls13_signature (input : & SignatureVerificationInput < '_ > , supported_schemes : & WebPkiSupportedAlgorithms ,) -> Result < HandshakeSignatureValid , Error > { if ! input . signature . scheme . supported_in_tls13 () { return Err (PeerMisbehaved :: SignedHandshakeWithUnadvertisedSigScheme . into ()) ; } let alg = supported_schemes . convert_scheme (input . signature . scheme) ? [0] ; match input . signer { SignerPublicKey :: X509 (cert_der) => { webpki :: EndEntityCert :: try_from (* cert_der) . and_then (| cert | { cert . verify_signature (alg , input . message , input . signature . signature ()) }) } SignerPublicKey :: RawPublicKey (spki) => webpki :: RawPublicKeyEntity :: try_from (* spki) . and_then (| rpk | rpk . verify_signature (alg , input . message , input . signature . signature ())) , } . map_err (pki_error) . map (| _ | HandshakeSignatureValid :: assertion ()) }
};
}
