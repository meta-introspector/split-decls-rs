// Generated macro for impl_657 (impl)
macro_rules! Depcrate_importsimpl_657 {
() => {
// Module: crate::imports
// Provides: {"impl_657"}
// Dependencies: {}
impl fmt :: Display for UseTree { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (i , segment) in self . path . iter () . enumerate () { if i != 0 { write ! (f , "::") ? ; } write ! (f , "{segment}") ? ; } Ok (()) } }
};
}
