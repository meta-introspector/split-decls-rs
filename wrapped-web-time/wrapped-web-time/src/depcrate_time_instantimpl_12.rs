// Generated macro for impl_12 (impl)
macro_rules! Depcrate_time_instantimpl_12 {
() => {
// Module: crate::time::instant
// Provides: {"impl_12"}
// Dependencies: {}
impl Sub < Self > for Instant { type Output = Duration ; # [doc = " Returns the amount of time elapsed from another instant to this one,"] # [doc = " or zero duration if that instant is later than this one."] fn sub (self , rhs : Self) -> Duration { self . duration_since (rhs) } }
};
}
