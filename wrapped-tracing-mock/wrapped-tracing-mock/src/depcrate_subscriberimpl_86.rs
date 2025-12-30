// Generated macro for impl_86 (impl)
macro_rules! Depcrate_subscriberimpl_86 {
() => {
// Module: crate::subscriber
// Provides: {"impl_86"}
// Dependencies: {}
impl From < & SpanState > for ActualSpan { fn from (span_state : & SpanState) -> Self { Self :: new (span_state . id . clone () , Some (span_state . meta)) } }
};
}
