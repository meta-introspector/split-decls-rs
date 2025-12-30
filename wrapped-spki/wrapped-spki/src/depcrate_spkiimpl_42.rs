// Generated macro for impl_42 (impl)
macro_rules! Depcrate_spkiimpl_42 {
() => {
// Module: crate::spki
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a , Params , Key > EncodeValue for SubjectPublicKeyInfo < Params , Key > where Params : Choice < 'a , Error = der :: Error > + Encode , Key : Encode , { fn value_len (& self) -> der :: Result < Length > { self . algorithm . encoded_len () ? + self . subject_public_key . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . algorithm . encode (writer) ? ; self . subject_public_key . encode (writer) ? ; Ok (()) } }
};
}
