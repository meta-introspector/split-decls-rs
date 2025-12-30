// Generated macro for impl_29 (impl)
macro_rules! Depcrate_aggregateimpl_29 {
() => {
// Module: crate::aggregate
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a > SamplePoints < 'a > { fn new < 'b : 'a , I : Iterator < Item = Event < 'a > > + DoubleEndedIterator + 'b > (events : I) -> Self { let mut rev_events = (Box :: new (events . rev () . filter (| e | ! e . payload . is_integer ())) as Box < dyn Iterator < Item = Event < 'a > > >) . peekable () ; SamplePoints { expected_thread_id : rev_events . peek () . map_or (0 , | event | event . thread_id) , rev_events , stack : vec ! [] , } } fn intervals (self) -> SampleIntervals < Self > { SampleIntervals :: new (self) } }
};
}
