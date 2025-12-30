// Generated macro for impl_262 (impl)
macro_rules! Depcrate_hedgeimpl_262 {
() => {
// Module: crate::hedge
// Provides: {"impl_262"}
// Dependencies: {}
impl latency :: Record for Histo { fn record (& mut self , latency : Duration) { let mut locked = self . lock () . unwrap () ; locked . write () . record (millis (latency)) . unwrap_or_else (| e | { error ! ("Failed to write to hedge histogram: {:?}" , e) ; }) } }
};
}
