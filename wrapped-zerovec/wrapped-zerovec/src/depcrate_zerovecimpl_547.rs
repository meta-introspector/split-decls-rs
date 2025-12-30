// Generated macro for impl_547 (impl)
macro_rules! Depcrate_zerovecimpl_547 {
() => {
// Module: crate::zerovec
// Provides: {"impl_547"}
// Dependencies: {}
impl < T > fmt :: Debug for ZeroVec < '_ , T > where T : AsULE + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ZeroVec([") ? ; let mut first = true ; for el in self . iter () { if ! first { write ! (f , ", ") ? ; } write ! (f , "{el:?}") ? ; first = false ; } write ! (f , "])") } }
};
}
