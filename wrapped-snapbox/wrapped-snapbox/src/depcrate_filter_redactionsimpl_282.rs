// Generated macro for impl_282 (impl)
macro_rules! Depcrate_filter_redactionsimpl_282 {
() => {
// Module: crate::filter::redactions
// Provides: {"impl_282"}
// Dependencies: {}
# [cfg (feature = "regex")] impl From < regex :: Regex > for RedactedValue { fn from (inner : regex :: Regex) -> Self { Self { inner : Some (RedactedValueInner :: Regex (inner)) , } } }
};
}
