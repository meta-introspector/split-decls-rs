// Generated macro for allocating (module)
macro_rules! Depcrate_spkiallocating {
() => {
// Module: crate::spki
// Provides: {"allocating"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod allocating { use super :: * ; use crate :: EncodePublicKey ; use der :: referenced :: * ; impl < 'a > RefToOwned < 'a > for SubjectPublicKeyInfoRef < 'a > { type Owned = SubjectPublicKeyInfoOwned ; fn ref_to_owned (& self) -> Self :: Owned { SubjectPublicKeyInfo { algorithm : self . algorithm . ref_to_owned () , subject_public_key : self . subject_public_key . ref_to_owned () , } } } impl OwnedToRef for SubjectPublicKeyInfoOwned { type Borrowed < 'a > = SubjectPublicKeyInfoRef < 'a > ; fn owned_to_ref (& self) -> Self :: Borrowed < '_ > { SubjectPublicKeyInfo { algorithm : self . algorithm . owned_to_ref () , subject_public_key : self . subject_public_key . owned_to_ref () , } } } impl SubjectPublicKeyInfoOwned { # [doc = " Create a [`SubjectPublicKeyInfoOwned`] from any object that implements"] # [doc = " [`EncodePublicKey`]."] pub fn from_key < T > (source : & T) -> Result < Self > where T : EncodePublicKey , { Ok (source . to_public_key_der () ? . decode_msg :: < Self > () ?) } } }
};
}
