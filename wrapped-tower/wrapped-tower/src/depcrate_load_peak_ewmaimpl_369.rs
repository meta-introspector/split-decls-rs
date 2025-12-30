// Generated macro for impl_369 (impl)
macro_rules! Depcrate_load_peak_ewmaimpl_369 {
() => {
// Module: crate::load::peak_ewma
// Provides: {"impl_369"}
// Dependencies: {}
impl RttEstimate { fn new (rtt_ns : f64) -> Self { debug_assert ! (0.0 < rtt_ns , "rtt must be positive") ; Self { rtt_ns , update_at : Instant :: now () , } } # [doc = " Decays the RTT estimate with a decay period of `decay_ns`."] fn decay (& mut self , decay_ns : f64) -> f64 { let now = Instant :: now () ; self . update (now , now , decay_ns) } # [doc = " Updates the Peak-EWMA RTT estimate."] # [doc = ""] # [doc = " The elapsed time from `sent_at` to `recv_at` is added"] fn update (& mut self , sent_at : Instant , recv_at : Instant , decay_ns : f64) -> f64 { debug_assert ! (sent_at <= recv_at , "recv_at={:?} after sent_at={:?}" , recv_at , sent_at) ; let rtt = nanos (recv_at . saturating_duration_since (sent_at)) ; let now = Instant :: now () ; debug_assert ! (self . update_at <= now , "update_at={:?} in the future" , self . update_at) ; self . rtt_ns = if self . rtt_ns < rtt { trace ! ("update peak rtt={}ms prior={}ms" , rtt / NANOS_PER_MILLI , self . rtt_ns / NANOS_PER_MILLI ,) ; rtt } else { let elapsed = nanos (now . saturating_duration_since (self . update_at)) ; let decay = (- elapsed / decay_ns) . exp () ; let recency = 1.0 - decay ; let next_estimate = (self . rtt_ns * decay) + (rtt * recency) ; trace ! ("update rtt={:03.0}ms decay={:06.0}ns; next={:03.0}ms" , rtt / NANOS_PER_MILLI , self . rtt_ns - next_estimate , next_estimate / NANOS_PER_MILLI ,) ; next_estimate } ; self . update_at = now ; self . rtt_ns } }
};
}
