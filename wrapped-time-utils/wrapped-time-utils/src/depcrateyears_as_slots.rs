// Generated macro for years_as_slots (function)
macro_rules! Depcrateyears_as_slots {
() => {
// Module: crate
// Provides: {"years_as_slots"}
// Dependencies: {}
# [doc = " from years to slots"] pub fn years_as_slots (years : f64 , tick_duration : & Duration , ticks_per_slot : u64) -> f64 { years * SECONDS_PER_YEAR * (1_000_000_000.0 / tick_duration . as_nanos () as f64) / ticks_per_slot as f64 }
};
}
