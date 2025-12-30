// Generated macro for impl_108 (impl)
macro_rules! Depcrate_locationimpl_108 {
() => {
// Module: crate::location
// Provides: {"impl_108"}
// Dependencies: {}
impl From < Range > for miette :: SourceSpan { fn from (value : Range) -> Self { let offset = value . start . offset ; let len = value . end . offset - offset ; (offset , len) . into () } }
};
}
