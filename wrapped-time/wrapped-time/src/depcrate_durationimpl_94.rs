// Generated macro for impl_94 (impl)
macro_rules! Depcrate_durationimpl_94 {
() => {
// Module: crate::duration
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (feature = "std")] impl Add < Duration > for SystemTime { type Output = Self ; # [inline] # [track_caller] fn add (self , duration : Duration) -> Self :: Output { if duration . is_zero () { self } else if duration . is_positive () { self + duration . unsigned_abs () } else { debug_assert ! (duration . is_negative ()) ; self - duration . unsigned_abs () } } }
};
}
