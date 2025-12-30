// Generated macro for impl_47 (impl)
macro_rules! Depcrate_spkiimpl_47 {
() => {
// Module: crate::spki
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a : 'k , 'k , Params , Key : 'k > TryFrom < & SubjectPublicKeyInfo < Params , Key > > for Document where Params : Choice < 'a , Error = der :: Error > + Encode , Key : Decode < 'a , Error = der :: Error > + Encode + FixedTag , BitStringRef < 'a > : From < & 'k Key > , { type Error = Error ; fn try_from (spki : & SubjectPublicKeyInfo < Params , Key >) -> Result < Document > { Ok (Self :: encode_msg (spki) ?) } }
};
}
