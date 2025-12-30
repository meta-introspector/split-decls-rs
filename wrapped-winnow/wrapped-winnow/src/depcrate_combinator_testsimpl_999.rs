// Generated macro for impl_999 (impl)
macro_rules! Depcrate_combinator_testsimpl_999 {
() => {
// Module: crate::combinator::tests
// Provides: {"impl_999"}
// Dependencies: {}
impl < I : Stream > ParserError < I > for CustomError { type Inner = Self ; fn from_input (_ : & I) -> Self { CustomError } fn into_inner (self) -> Result < Self :: Inner , Self > { Ok (self) } }
};
}
