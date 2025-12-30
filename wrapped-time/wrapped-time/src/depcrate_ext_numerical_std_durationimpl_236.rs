// Generated macro for impl_236 (impl)
macro_rules! Depcrate_ext_numerical_std_durationimpl_236 {
() => {
// Module: crate::ext::numerical_std_duration
// Provides: {"impl_236"}
// Dependencies: {}
impl NumericalStdDuration for u64 { # [inline] fn std_nanoseconds (self) -> StdDuration { StdDuration :: from_nanos (self) } # [inline] fn std_microseconds (self) -> StdDuration { StdDuration :: from_micros (self) } # [inline] fn std_milliseconds (self) -> StdDuration { StdDuration :: from_millis (self) } # [inline] fn std_seconds (self) -> StdDuration { StdDuration :: from_secs (self) } # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn std_minutes (self) -> StdDuration { StdDuration :: from_secs (self . checked_mul (Second :: per_t (Minute)) . expect ("overflow constructing `time::Duration`") ,) } # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn std_hours (self) -> StdDuration { StdDuration :: from_secs (self . checked_mul (Second :: per_t (Hour)) . expect ("overflow constructing `time::Duration`") ,) } # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn std_days (self) -> StdDuration { StdDuration :: from_secs (self . checked_mul (Second :: per_t (Day)) . expect ("overflow constructing `time::Duration`") ,) } # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn std_weeks (self) -> StdDuration { StdDuration :: from_secs (self . checked_mul (Second :: per_t (Week)) . expect ("overflow constructing `time::Duration`") ,) } }
};
}
