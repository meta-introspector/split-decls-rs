// Generated macro for impl_81 (impl)
macro_rules! Depcrate_patimpl_81 {
() => {
// Module: crate::pat
// Provides: {"impl_81"}
// Dependencies: {}
# [doc = " This is best effort and not good enough for a `Display` impl."] impl < Cx : PatCx > fmt :: Debug for DeconstructedPat < Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut fields : Vec < _ > = (0 .. self . arity) . map (| _ | PatOrWild :: Wild) . collect () ; for ipat in self . iter_fields () { fields [ipat . idx] = PatOrWild :: Pat (& ipat . pat) ; } self . ctor () . fmt_fields (f , self . ty () , fields . into_iter ()) } }
};
}
