// Generated macro for impl_26 (impl)
macro_rules! Depcrate_aggregateimpl_26 {
() => {
// Module: crate::aggregate
// Provides: {"impl_26"}
// Dependencies: {}
impl < E > SamplePoint < E > { fn event (& self) -> & E { match self { SamplePoint :: Start (event) | SamplePoint :: End (event) | SamplePoint :: Instant (event) => { event } } } fn map_event < E2 > (self , f : impl FnOnce (E) -> E2) -> SamplePoint < E2 > { match self { SamplePoint :: Start (event) => SamplePoint :: Start (f (event)) , SamplePoint :: End (event) => SamplePoint :: End (f (event)) , SamplePoint :: Instant (event) => SamplePoint :: Instant (f (event)) , } } }
};
}
