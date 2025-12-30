// Generated macro for SamplePoints (struct)
macro_rules! Depcrate_aggregateSamplePoints {
() => {
// Module: crate::aggregate
// Provides: {"SamplePoints"}
// Dependencies: {}
struct SamplePoints < 'a > { # [doc = " This analysis only works with deterministic runs, which precludes parallelism,"] # [doc = " so we just have to find the *only* thread's ID and require there is no other."] expected_thread_id : u32 , # [doc = " Reversed events that do not contain integer payloads"] rev_events : std :: iter :: Peekable < Box < dyn Iterator < Item = Event < 'a > > + 'a > > , stack : Vec < Event < 'a > > , }
};
}
