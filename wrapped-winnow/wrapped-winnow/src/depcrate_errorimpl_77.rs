// Generated macro for impl_77 (impl)
macro_rules! Depcrate_errorimpl_77 {
() => {
// Module: crate::error
// Provides: {"impl_77"}
// Dependencies: {}
impl < I : Stream + Clone > ParserError < I > for InputError < I > { type Inner = Self ; # [inline] fn from_input (input : & I) -> Self { Self { input : input . clone () , } } # [inline (always)] fn into_inner (self) -> Result < Self :: Inner , Self > { Ok (self) } }
};
}
