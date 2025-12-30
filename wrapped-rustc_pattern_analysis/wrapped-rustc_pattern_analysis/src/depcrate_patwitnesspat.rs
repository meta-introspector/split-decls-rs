// Generated macro for WitnessPat (struct)
macro_rules! Depcrate_patWitnessPat {
() => {
// Module: crate::pat
// Provides: {"WitnessPat"}
// Dependencies: {}
# [doc = " Same idea as `DeconstructedPat`, except this is a fictitious pattern built up for diagnostics"] # [doc = " purposes. As such they don't use interning and can be cloned."] pub struct WitnessPat < Cx : PatCx > { ctor : Constructor < Cx > , pub (crate) fields : Vec < WitnessPat < Cx > > , ty : Cx :: Ty , }
};
}
