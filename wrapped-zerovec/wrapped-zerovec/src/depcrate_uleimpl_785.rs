// Generated macro for impl_785 (impl)
macro_rules! Depcrate_uleimpl_785 {
() => {
// Module: crate::ule
// Provides: {"impl_785"}
// Dependencies: {}
impl fmt :: Display for UleError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { UleError :: InvalidLength { ty , len } => { write ! (f , "Invalid length {len} for slice of type {ty}") } UleError :: ParseError { ty } => { write ! (f , "Could not parse bytes to slice of type {ty}") } } } }
};
}
