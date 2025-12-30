// Generated macro for impl_106 (impl)
macro_rules! Depcrate_replaceimpl_106 {
() => {
// Module: crate::replace
// Provides: {"impl_106"}
// Dependencies: {}
impl < I : Iterator < Item = char > + Clone > fmt :: Display for Replacements < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for c in self . clone () { f . write_char (c) ? ; } Ok (()) } }
};
}
