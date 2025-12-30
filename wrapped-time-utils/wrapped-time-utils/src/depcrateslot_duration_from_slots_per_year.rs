// Generated macro for slot_duration_from_slots_per_year (function)
macro_rules! Depcrateslot_duration_from_slots_per_year {
() => {
// Module: crate
// Provides: {"slot_duration_from_slots_per_year"}
// Dependencies: {}
# [doc = " From slots per year to slot duration"] pub fn slot_duration_from_slots_per_year (slots_per_year : f64) -> Duration { let slot_in_ns = if slots_per_year != 0.0 { (SECONDS_PER_YEAR * 1_000_000_000.0) / slots_per_year } else { 0.0 } ; Duration :: from_nanos (slot_in_ns as u64) }
};
}
