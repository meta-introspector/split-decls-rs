// Generated macro for impl_32 (impl)
macro_rules! Depcrate_aggregateimpl_32 {
() => {
// Module: crate::aggregate
// Provides: {"impl_32"}
// Dependencies: {}
impl < E > SampleInterval < E > { fn map_event < E2 > (self , f : impl Copy + FnOnce (E) -> E2) -> SampleInterval < E2 > { SampleInterval { start : self . start . map_event (f) , end : self . end . map_event (f) , } } }
};
}
