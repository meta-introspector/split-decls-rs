// Generated macro for impl_43 (impl)
macro_rules! Depcrate_spkiimpl_43 {
() => {
// Module: crate::spki
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a , Params , Key > Sequence < 'a > for SubjectPublicKeyInfo < Params , Key > where Params : Choice < 'a , Error = der :: Error > + Encode , Key : Decode < 'a , Error = der :: Error > + Encode + FixedTag , { }
};
}
