// Generated macro for impl_9 (impl)
macro_rules! Depcrate_time_instantimpl_9 {
() => {
// Module: crate::time::instant
// Provides: {"impl_9"}
// Dependencies: {}
impl Add < Duration > for Instant { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This function may panic if the resulting point in time cannot be"] # [doc = " represented by the underlying data structure. See"] # [doc = " [`Instant::checked_add`] for a version without panic."] fn add (self , rhs : Duration) -> Self { self . checked_add (rhs) . expect ("overflow when adding duration to instant") } }
};
}
