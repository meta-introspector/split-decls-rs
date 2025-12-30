// Generated macro for SpanRef (struct)
macro_rules! Depcrate_registrySpanRef {
() => {
// Module: crate::registry
// Provides: {"SpanRef"}
// Dependencies: {}
# [doc = " A reference to [span data] and the associated [registry]."] # [doc = ""] # [doc = " This type implements all the same methods as [`SpanData`], and provides"] # [doc = " additional methods for querying the registry based on values from the span."] # [doc = ""] # [doc = " [registry]: LookupSpan"] # [derive (Debug)] pub struct SpanRef < 'a , R : LookupSpan < 'a > > { registry : & 'a R , data : R :: Data , # [cfg (feature = "registry")] filter : FilterId , }
};
}
