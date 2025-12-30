// Generated macro for impl_key_identifier (macro)
macro_rules! Depcrate_macrosimpl_key_identifier {
() => {
// Module: crate::macros
// Provides: {"impl_key_identifier"}
// Dependencies: {}
# [doc = " Implements conversions between [`spki::SubjectPublicKeyInfo`] and [`SubjectKeyIdentifier`] or [`AuthorityKeyIdentifier`]"] macro_rules ! impl_key_identifier { ($ newtype : ty , $ out : expr) => { # [cfg (feature = "builder")] mod builder_key_identifier { use super ::*; use der :: asn1 :: OctetString ; use sha1 :: { Digest , Sha1 } ; use spki :: SubjectPublicKeyInfoRef ; impl <'a > TryFrom < SubjectPublicKeyInfoRef <'a >> for $ newtype { type Error = der :: Error ; fn try_from (issuer : SubjectPublicKeyInfoRef <'a >) -> Result < Self , Self :: Error > { let result = Sha1 :: digest (issuer . subject_public_key . raw_bytes ()) ; $ out (result . as_slice ()) } } } } ; }
};
}
