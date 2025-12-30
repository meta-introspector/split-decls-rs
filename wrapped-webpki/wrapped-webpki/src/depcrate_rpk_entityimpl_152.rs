// Generated macro for impl_152 (impl)
macro_rules! Depcrate_rpk_entityimpl_152 {
() => {
// Module: crate::rpk_entity
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a SubjectPublicKeyInfoDer < 'a > > for RawPublicKeyEntity < 'a > { type Error = Error ; # [doc = " Parse the ASN.1 DER-encoded SPKI encoding of the raw public key `spki`."] # [doc = " Since we are parsing a raw public key, we first strip the outer sequence tag."] fn try_from (spki : & 'a SubjectPublicKeyInfoDer < 'a >) -> Result < Self , Self :: Error > { let input = untrusted :: Input :: from (spki . as_ref ()) ; let spki = input . read_all (Error :: TrailingData (DerTypeId :: SubjectPublicKeyInfo) , | reader | { let untagged_spki = der :: expect_tag (reader , der :: Tag :: Sequence) ? ; der :: read_all :: < SubjectPublicKeyInfo < '_ > > (untagged_spki) ? ; Ok (untagged_spki) } ,) ? ; Ok (Self { inner : spki }) } }
};
}
