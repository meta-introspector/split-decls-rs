// Generated macro for impl_45 (impl)
macro_rules! Depcrate_spkiimpl_45 {
() => {
// Module: crate::spki
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a , Params , Key > ValueOrd for SubjectPublicKeyInfo < Params , Key > where Params : Choice < 'a , Error = der :: Error > + DerOrd + Encode , Key : ValueOrd , { fn value_cmp (& self , other : & Self) -> der :: Result < Ordering > { match self . algorithm . der_cmp (& other . algorithm) ? { Ordering :: Equal => self . subject_public_key . value_cmp (& other . subject_public_key) , other => Ok (other) , } } }
};
}
