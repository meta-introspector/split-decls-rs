// Generated macro for impl_44 (impl)
macro_rules! Depcrate_spkiimpl_44 {
() => {
// Module: crate::spki
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , Params , Key > TryFrom < & 'a [u8] > for SubjectPublicKeyInfo < Params , Key > where Params : Choice < 'a , Error = der :: Error > + Encode , Key : Decode < 'a , Error = der :: Error > + Encode + FixedTag , { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self > { Ok (Self :: from_der (bytes) ?) } }
};
}
