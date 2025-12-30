// Generated macro for impl_46 (impl)
macro_rules! Depcrate_spkiimpl_46 {
() => {
// Module: crate::spki
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a : 'k , 'k , Params , Key : 'k > TryFrom < SubjectPublicKeyInfo < Params , Key > > for Document where Params : Choice < 'a , Error = der :: Error > + Encode , Key : Decode < 'a , Error = der :: Error > + Encode + FixedTag , BitStringRef < 'a > : From < & 'k Key > , { type Error = Error ; fn try_from (spki : SubjectPublicKeyInfo < Params , Key >) -> Result < Document > { Self :: try_from (& spki) } }
};
}
