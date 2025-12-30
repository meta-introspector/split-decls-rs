// Generated macro for impl_1858 (impl)
macro_rules! Depcrate_verifybenchimpl_1858 {
() => {
// Module: crate::verifybench
// Provides: {"impl_1858"}
// Dependencies: {}
impl Context { fn new (provider : CryptoProvider , domain : & 'static str , certs : & [& 'static [u8]]) -> Self { let mut roots = RootCertStore :: empty () ; roots . extend (webpki_roots :: TLS_SERVER_ROOTS . iter () . cloned () ,) ; Self { server_name : domain . try_into () . unwrap () , chain : certs . iter () . copied () . map (| bytes | CertificateDer :: from (bytes . to_vec ())) . collect () , now : UnixTime :: since_unix_epoch (Duration :: from_secs (1_746_605_469)) , verifier : WebPkiServerVerifier :: new_without_revocation (roots , provider . signature_verification_algorithms ,) , } } fn verify_once (& self) { const OCSP_RESPONSE : & [u8] = & [] ; self . verifier . verify_identity (& ServerIdentity { identity : & Identity :: X509 (CertificateIdentity { end_entity : self . chain [0] . clone () , intermediates : self . chain [1 ..] . to_vec () , }) , server_name : & self . server_name , ocsp_response : OCSP_RESPONSE , now : self . now , }) . unwrap () ; } }
};
}
