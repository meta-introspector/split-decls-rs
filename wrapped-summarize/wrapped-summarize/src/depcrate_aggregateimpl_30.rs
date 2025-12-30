// Generated macro for impl_30 (impl)
macro_rules! Depcrate_aggregateimpl_30 {
() => {
// Module: crate::aggregate
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > BackwardsIterator for SamplePoints < 'a > { type Item = SamplePoint < WithParent < Event < 'a > > > ; fn next_back (& mut self) -> Option < Self :: Item > { let sample_point = match self . rev_events . peek () { Some (peeked_event) => { assert ! (! peeked_event . payload . is_integer () , "Integer events accidentally included in `SamplePoints` events") ; assert_eq ! (peeked_event . thread_id , self . expected_thread_id , "more than one thread is not supported in `summarize aggregate`") ; match self . stack . last () { Some (top_event) if ! top_event . contains (peeked_event) => { SamplePoint :: Start (self . stack . pop () . unwrap ()) } _ => { let event = self . rev_events . next () . unwrap () ; match event . payload { EventPayload :: Timestamp (Timestamp :: Interval { .. }) => { self . stack . push (event . clone ()) ; SamplePoint :: End (event) } EventPayload :: Timestamp (Timestamp :: Instant (_)) => { SamplePoint :: Instant (event) } EventPayload :: Integer (_) => { unreachable ! () } } } } } None => SamplePoint :: Start (self . stack . pop () ?) , } ; let parent = match sample_point { SamplePoint :: End (_) => { if self . stack . len () >= 2 { Some (& self . stack [self . stack . len () - 2]) } else { None } } SamplePoint :: Start (_) | SamplePoint :: Instant (_) => self . stack . last () , } ; Some (sample_point . map_event (| this | WithParent { this , parent : parent . cloned () , })) } }
};
}
