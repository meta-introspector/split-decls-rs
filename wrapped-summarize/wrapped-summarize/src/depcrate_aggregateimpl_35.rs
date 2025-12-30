// Generated macro for impl_35 (impl)
macro_rules! Depcrate_aggregateimpl_35 {
() => {
// Module: crate::aggregate
// Provides: {"impl_35"}
// Dependencies: {}
impl < I : BackwardsIterator > SampleIntervals < I > { fn new (mut sample_points : I) -> Self { SampleIntervals { last_sample_point : sample_points . next_back () , sample_points , } } }
};
}
