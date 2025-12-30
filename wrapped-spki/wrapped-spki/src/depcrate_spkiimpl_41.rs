// Generated macro for impl_41 (impl)
macro_rules! Depcrate_spkiimpl_41 {
() => {
// Module: crate::spki
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a , Params , Key > DecodeValue < 'a > for SubjectPublicKeyInfo < Params , Key > where Params : Choice < 'a , Error = der :: Error > + Encode , Key : Decode < 'a , Error = der :: Error > , { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { Ok (Self { algorithm : reader . decode () ? , subject_public_key : Key :: decode (reader) ? , }) } }
};
}
