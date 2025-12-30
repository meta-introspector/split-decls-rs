// Generated macro for impl_264 (impl)
macro_rules! Depcrate_hedgeimpl_264 {
() => {
// Module: crate::hedge
// Provides: {"impl_264"}
// Dependencies: {}
impl < Request > delay :: Policy < Request > for DelayPolicy { fn delay (& self , _req : & Request) -> Duration { let mut locked = self . histo . lock () . unwrap () ; let millis = locked . read () . value_at_quantile (self . latency_percentile . into ()) ; Duration :: from_millis (millis) } }
};
}
