// Generated macro for Record (trait)
macro_rules! Depcrate_hedge_latencyRecord {
() => {
// Module: crate::hedge::latency
// Provides: {"Record"}
// Dependencies: {}
# [doc = " Record is the interface for accepting request latency measurements.  When"] # [doc = " a request completes, record is called with the elapsed duration between"] # [doc = " when the service was called and when the future completed."] pub trait Record { fn record (& mut self , latency : Duration) ; }
};
}
