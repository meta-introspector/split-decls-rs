// Generated macro for impl_28 (impl)
macro_rules! Depcrate_field_delimitedimpl_28 {
() => {
// Module: crate::field::delimited
// Provides: {"impl_28"}
// Dependencies: {}
impl < D , V > Delimited < D , V > { # [doc = " Returns a new [`MakeVisitor`] implementation that wraps `inner` so that"] # [doc = " it will format each visited field separated by the provided `delimiter`."] # [doc = ""] # [doc = " [`MakeVisitor`]: super::MakeVisitor"] pub fn new (delimiter : D , inner : V) -> Self { Self { delimiter , inner } } }
};
}
