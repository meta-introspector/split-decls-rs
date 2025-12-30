// Generated macro for impl_42 (impl)
macro_rules! Depcrate_aggregateimpl_42 {
() => {
// Module: crate::aggregate
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a , I : BackwardsIterator < Item = SampleInterval < WithParent < Event < 'a > > > > > BackwardsIterator for AggregatedSampleIntervals < I > { type Item = AggregatedSampleInterval < 'a > ; fn next_back (& mut self) -> Option < Self :: Item > { match self . sample_intervals_per_profile . get_mut (0) ? . next_back () { Some (interval) => { let first_duration = interval . duration () ; let descriptions = interval . map_event (WithParent :: < EventDescription < '_ > > :: from) ; let mut durations_across_profiles = std :: iter :: once (first_duration) . chain (self . sample_intervals_per_profile [1 ..] . iter_mut () . map (| it | { let interval = it . next_back () . expect ("`summarize aggregate` requires identical sequences of events") ; let duration = interval . duration () ; assert_eq ! (descriptions , interval . map_event (WithParent ::< EventDescription <'_ >>:: from) , "`summarize aggregate` requires identical sequences of events") ; duration }) ,) ; let (mut min_duration , mut max_duration) = { let first = durations_across_profiles . next () . unwrap () ; (first , first) } ; for duration in durations_across_profiles { min_duration = min_duration . min (duration) ; max_duration = max_duration . max (duration) ; } Some (AggregatedSampleInterval { descriptions , min_duration , duration_variance : Variance { range_size : max_duration - min_duration , } , }) } None => { for leftover_intervals in self . sample_intervals_per_profile . iter_mut () { assert_eq ! (leftover_intervals . next_back () , None , "`summarize aggregate` requires identical sequences of events") ; } None } } } }
};
}
