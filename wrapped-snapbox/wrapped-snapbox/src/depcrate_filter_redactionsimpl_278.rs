// Generated macro for impl_278 (impl)
macro_rules! Depcrate_filter_redactionsimpl_278 {
() => {
// Module: crate::filter::redactions
// Provides: {"impl_278"}
// Dependencies: {}
impl From < Cow < 'static , str > > for RedactedValue { fn from (inner : Cow < 'static , str >) -> Self { match inner { Cow :: Borrowed (s) => s . into () , Cow :: Owned (s) => s . into () , } } }
};
}
