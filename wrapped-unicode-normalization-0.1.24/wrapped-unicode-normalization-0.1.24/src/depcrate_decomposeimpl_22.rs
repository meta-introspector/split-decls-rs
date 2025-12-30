// Generated macro for impl_22 (impl)
macro_rules! Depcrate_decomposeimpl_22 {
() => {
// Module: crate::decompose
// Provides: {"impl_22"}
// Dependencies: {}
impl < I : Iterator < Item = char > + Clone > fmt :: Display for Decompositions < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for c in self . clone () { f . write_char (c) ? ; } Ok (()) } }
};
}
