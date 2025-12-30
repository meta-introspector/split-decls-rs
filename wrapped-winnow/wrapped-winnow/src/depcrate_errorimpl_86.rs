// Generated macro for impl_86 (impl)
macro_rules! Depcrate_errorimpl_86 {
() => {
// Module: crate::error
// Provides: {"impl_86"}
// Dependencies: {}
impl < I : Stream > ParserError < I > for EmptyError { type Inner = Self ; # [inline (always)] fn from_input (_ : & I) -> Self { Self } # [inline (always)] fn into_inner (self) -> Result < Self :: Inner , Self > { Ok (self) } }
};
}
