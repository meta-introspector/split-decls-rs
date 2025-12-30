// Generated macro for impl_92 (impl)
macro_rules! Depcrate_errorimpl_92 {
() => {
// Module: crate::error
// Provides: {"impl_92"}
// Dependencies: {}
impl < I : Stream > ParserError < I > for () { type Inner = Self ; # [inline] fn from_input (_ : & I) -> Self { } # [inline (always)] fn into_inner (self) -> Result < Self :: Inner , Self > { Ok (self) } }
};
}
