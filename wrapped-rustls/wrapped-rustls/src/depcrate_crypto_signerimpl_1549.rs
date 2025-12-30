// Generated macro for impl_1549 (impl)
macro_rules! Depcrate_crypto_signerimpl_1549 {
() => {
// Module: crate::crypto::signer
// Provides: {"impl_1549"}
// Dependencies: {}
impl < 'a > Codec < 'a > for Identity < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { match self { Self :: X509 (certificates) => { 0u8 . encode (bytes) ; certificates . end_entity . encode (bytes) ; certificates . intermediates . encode (bytes) ; } Self :: RawPublicKey (spki) => { 1u8 . encode (bytes) ; spki . encode (bytes) ; } } } fn read (reader : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { match u8 :: read (reader) ? { 0 => Ok (Self :: X509 (CertificateIdentity { end_entity : CertificateDer :: read (reader) ? . into_owned () , intermediates : Vec :: < CertificateDer < '_ > > :: read (reader) ? . into_iter () . collect () , })) , 1 => Ok (Self :: RawPublicKey (SubjectPublicKeyInfoDer :: read (reader) ? . into_owned () ,)) , _ => Err (InvalidMessage :: UnexpectedMessage ("invalid PeerIdentity discriminant" ,)) , } } }
};
}
