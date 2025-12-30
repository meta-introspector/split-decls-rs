// Generated macro for impl_36 (impl)
macro_rules! Depcrate_aggregateimpl_36 {
() => {
// Module: crate::aggregate
// Provides: {"impl_36"}
// Dependencies: {}
impl < E : Clone , I : BackwardsIterator < Item = SamplePoint < E > > > BackwardsIterator for SampleIntervals < I > { type Item = SampleInterval < E > ; fn next_back (& mut self) -> Option < Self :: Item > { let start = self . sample_points . next_back () ? ; let end = self . last_sample_point . replace (start . clone ()) ? ; Some (SampleInterval { start , end }) } }
};
}
