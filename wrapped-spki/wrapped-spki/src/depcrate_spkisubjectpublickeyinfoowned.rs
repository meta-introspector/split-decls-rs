// Generated macro for SubjectPublicKeyInfoOwned (type)
macro_rules! Depcrate_spkiSubjectPublicKeyInfoOwned {
() => {
// Module: crate::spki
// Provides: {"SubjectPublicKeyInfoOwned"}
// Dependencies: {}
# [doc = " [`SubjectPublicKeyInfo`] with [`Any`] algorithm parameters, and [`BitString`] params."] # [doc = ""] # [doc = " This is the owning-pendant to [`SubjectPublicKeyInfoRef`]."] # [cfg (feature = "alloc")] pub type SubjectPublicKeyInfoOwned = SubjectPublicKeyInfo < Any , BitString > ;
};
}
