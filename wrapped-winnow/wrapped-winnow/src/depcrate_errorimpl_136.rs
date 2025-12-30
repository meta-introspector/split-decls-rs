// Generated macro for impl_136 (impl)
macro_rules! Depcrate_errorimpl_136 {
() => {
// Module: crate::error
// Provides: {"impl_136"}
// Dependencies: {}
impl < I : AsBStr , E > ParseError < I , E > { # [doc = " The byte indices for the `char` at [`ParseError::offset`]"] # [inline] pub fn char_span (& self) -> core :: ops :: Range < usize > { char_boundary (self . input . as_bstr () , self . offset ()) } }
};
}
