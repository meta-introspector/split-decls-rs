// Generated macro for impl_537 (impl)
macro_rules! Depcrate_retry_backoffimpl_537 {
() => {
// Module: crate::retry::backoff
// Provides: {"impl_537"}
// Dependencies: {}
impl < R : Rng > ExponentialBackoff < R > { fn base (& self) -> time :: Duration { debug_assert ! (self . min <= self . max , "maximum backoff must not be less than minimum backoff") ; debug_assert ! (self . max > time :: Duration :: from_millis (0) , "Maximum backoff must be non-zero") ; self . min . checked_mul (2_u32 . saturating_pow (self . iterations)) . unwrap_or (self . max) . min (self . max) } # [doc = " Returns a random, uniform duration on `[0, base*self.jitter]` no greater"] # [doc = " than `self.max`."] fn jitter (& mut self , base : time :: Duration) -> time :: Duration { if self . jitter == 0.0 { time :: Duration :: default () } else { let jitter_factor = self . rng . next_f64 () ; debug_assert ! (jitter_factor > 0.0 , "rng returns values between 0.0 and 1.0") ; let rand_jitter = jitter_factor * self . jitter ; let secs = (base . as_secs () as f64) * rand_jitter ; let nanos = (base . subsec_nanos () as f64) * rand_jitter ; let remaining = self . max - base ; time :: Duration :: new (secs as u64 , nanos as u32) . min (remaining) } } }
};
}
