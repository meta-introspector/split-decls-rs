// Generated macro for impl_33 (impl)
macro_rules! Depcrate_aggregateimpl_33 {
() => {
// Module: crate::aggregate
// Provides: {"impl_33"}
// Dependencies: {}
impl SampleInterval < WithParent < Event < '_ > > > { fn duration (& self) -> Duration { self . end . timestamp () . duration_since (self . start . timestamp ()) . unwrap () } }
};
}
