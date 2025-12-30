// Generated macro for impl_41 (impl)
macro_rules! Depcrate_aggregateimpl_41 {
() => {
// Module: crate::aggregate
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a , I : BackwardsIterator < Item = SampleInterval < WithParent < Event < 'a > > > > > AggregatedSampleIntervals < I > { fn new (sample_intervals_per_profile : impl Iterator < Item = I >) -> Self { AggregatedSampleIntervals { sample_intervals_per_profile : sample_intervals_per_profile . collect () , } } }
};
}
