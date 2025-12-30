// Generated macro for impl_98 (impl)
macro_rules! Depcrate_recomposeimpl_98 {
() => {
// Module: crate::recompose
// Provides: {"impl_98"}
// Dependencies: {}
impl < I : Iterator < Item = char > + Clone > fmt :: Display for Recompositions < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for c in self . clone () { f . write_char (c) ? ; } Ok (()) } }
};
}
