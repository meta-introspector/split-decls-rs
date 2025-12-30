// Generated macro for impl_105 (impl)
macro_rules! Depcrate_layerimpl_105 {
() => {
// Module: crate::layer
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'a , S > From < & SpanRef < 'a , S > > for ActualSpan where S : LookupSpan < 'a > , { fn from (span_ref : & SpanRef < 'a , S >) -> Self { Self :: new (span_ref . id () , Some (span_ref . metadata ())) } }
};
}
