// Generated macro for impl_3410 (impl)
macro_rules! Depcrate_timeimpl_3410 {
() => {
// Module: crate::time
// Provides: {"impl_3410"}
// Dependencies: {}
# [stable (feature = "time2" , since = "1.8.0")] impl Add < Duration > for Instant { type Output = Instant ; # [doc = " # Panics"] # [doc = ""] # [doc = " This function may panic if the resulting point in time cannot be represented by the"] # [doc = " underlying data structure. See [`Instant::checked_add`] for a version without panic."] fn add (self , other : Duration) -> Instant { self . checked_add (other) . expect ("overflow when adding duration to instant") } }
};
}
