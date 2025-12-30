// Generated macro for impl_93 (impl)
macro_rules! Depcrate_patimpl_93 {
() => {
// Module: crate::pat
// Provides: {"impl_93"}
// Dependencies: {}
# [doc = " This is best effort and not good enough for a `Display` impl."] impl < Cx : PatCx > fmt :: Debug for WitnessPat < Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . ctor () . fmt_fields (f , self . ty () , self . fields . iter ()) } }
};
}
