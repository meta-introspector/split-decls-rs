// Generated macro for impl_84 (impl)
macro_rules! Depcrate_patimpl_84 {
() => {
// Module: crate::pat
// Provides: {"impl_84"}
// Dependencies: {}
# [doc = " Delegate to `uid`."] impl < Cx : PatCx > std :: hash :: Hash for DeconstructedPat < Cx > { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . uid . hash (state) ; } }
};
}
