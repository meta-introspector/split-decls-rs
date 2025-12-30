// Generated macro for impl_1174 (impl)
macro_rules! Depcrate_patternsimpl_1174 {
() => {
// Module: crate::patterns
// Provides: {"impl_1174"}
// Dependencies: {}
impl < 'a > Spanned for TuplePatField < 'a > { fn span (& self) -> Span { match * self { TuplePatField :: Pat (p) => p . span () , TuplePatField :: Dotdot (span) => span , } } }
};
}
