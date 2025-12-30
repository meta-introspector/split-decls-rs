// Generated macro for SubjectPublicKeyInfoRef (type)
macro_rules! Depcrate_spkiSubjectPublicKeyInfoRef {
() => {
// Module: crate::spki
// Provides: {"SubjectPublicKeyInfoRef"}
// Dependencies: {}
# [doc = " [`SubjectPublicKeyInfo`] with [`AnyRef`] algorithm parameters, and [`BitStringRef`] params."] # [doc = ""] # [doc = " This is the borrowing-pendant to [`SubjectPublicKeyInfoOwned`]."] pub type SubjectPublicKeyInfoRef < 'a > = SubjectPublicKeyInfo < AnyRef < 'a > , BitStringRef < 'a > > ;
};
}
