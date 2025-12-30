// Generated macro for impl_275 (impl)
macro_rules! Depcrate_filter_redactionsimpl_275 {
() => {
// Module: crate::filter::redactions
// Provides: {"impl_275"}
// Dependencies: {}
impl From < & 'static str > for RedactedValue { fn from (inner : & 'static str) -> Self { if inner . is_empty () { Self { inner : None } } else { Self { inner : Some (RedactedValueInner :: Str (inner)) , } } } }
};
}
