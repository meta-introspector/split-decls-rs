// Generated macro for time_stamp_to_duration (function)
macro_rules! Depcrate_time_instanttime_stamp_to_duration {
() => {
// Module: crate::time::instant
// Provides: {"time_stamp_to_duration"}
// Dependencies: {}
# [doc = " Converts a `DOMHighResTimeStamp` to a [`Duration`]."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Keep in mind that like [`Duration::from_secs_f64()`] this doesn't do perfect"] # [doc = " rounding."] # [allow (clippy :: as_conversions , clippy :: cast_possible_truncation , clippy :: cast_sign_loss)] fn time_stamp_to_duration (time_stamp : f64) -> Duration { let time_stamp = F64 (time_stamp) ; Duration :: from_millis (time_stamp . trunc () as u64) + Duration :: from_nanos (F64 (time_stamp . fract () * 1.0e6) . internal_round_ties_even () as u64) }
};
}
