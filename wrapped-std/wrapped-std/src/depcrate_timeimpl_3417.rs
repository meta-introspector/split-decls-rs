// Generated macro for impl_3417 (impl)
macro_rules! Depcrate_timeimpl_3417 {
() => {
// Module: crate::time
// Provides: {"impl_3417"}
// Dependencies: {}
# [stable (feature = "time2" , since = "1.8.0")] impl Add < Duration > for SystemTime { type Output = SystemTime ; # [doc = " # Panics"] # [doc = ""] # [doc = " This function may panic if the resulting point in time cannot be represented by the"] # [doc = " underlying data structure. See [`SystemTime::checked_add`] for a version without panic."] fn add (self , dur : Duration) -> SystemTime { self . checked_add (dur) . expect ("overflow when adding duration to instant") } }
};
}
