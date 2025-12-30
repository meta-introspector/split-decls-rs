// Generated macro for impl_40 (impl)
macro_rules! Depcrate_time_system_timeimpl_40 {
() => {
// Module: crate::time::system_time
// Provides: {"impl_40"}
// Dependencies: {}
impl Add < Duration > for SystemTime { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This function may panic if the resulting point in time cannot be"] # [doc = " represented by the underlying data structure. See"] # [doc = " [`SystemTime::checked_add`] for a version without panic."] fn add (self , rhs : Duration) -> Self { self . checked_add (rhs) . expect ("overflow when adding duration to instant") } }
};
}
