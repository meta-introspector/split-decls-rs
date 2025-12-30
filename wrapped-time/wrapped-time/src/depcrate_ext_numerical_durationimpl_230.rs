// Generated macro for impl_230 (impl)
macro_rules! Depcrate_ext_numerical_durationimpl_230 {
() => {
// Module: crate::ext::numerical_duration
// Provides: {"impl_230"}
// Dependencies: {}
impl NumericalDuration for f64 { # [inline] fn nanoseconds (self) -> Duration { Duration :: nanoseconds (self as i64) } # [inline] fn microseconds (self) -> Duration { Duration :: nanoseconds ((self * Nanosecond :: per_t :: < Self > (Microsecond)) as i64) } # [inline] fn milliseconds (self) -> Duration { Duration :: nanoseconds ((self * Nanosecond :: per_t :: < Self > (Millisecond)) as i64) } # [inline] fn seconds (self) -> Duration { Duration :: nanoseconds ((self * Nanosecond :: per_t :: < Self > (Second)) as i64) } # [inline] # [track_caller] fn minutes (self) -> Duration { Duration :: nanoseconds ((self * Nanosecond :: per_t :: < Self > (Minute)) as i64) } # [inline] # [track_caller] fn hours (self) -> Duration { Duration :: nanoseconds ((self * Nanosecond :: per_t :: < Self > (Hour)) as i64) } # [inline] # [track_caller] fn days (self) -> Duration { Duration :: nanoseconds ((self * Nanosecond :: per_t :: < Self > (Day)) as i64) } # [inline] # [track_caller] fn weeks (self) -> Duration { Duration :: nanoseconds ((self * Nanosecond :: per_t :: < Self > (Week)) as i64) } }
};
}
