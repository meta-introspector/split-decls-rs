// Generated macro for impl_276 (impl)
macro_rules! Depcrate_filter_redactionsimpl_276 {
() => {
// Module: crate::filter::redactions
// Provides: {"impl_276"}
// Dependencies: {}
impl From < String > for RedactedValue { fn from (inner : String) -> Self { if inner . is_empty () { Self { inner : None } } else { Self { inner : Some (RedactedValueInner :: String (inner)) , } } } }
};
}
