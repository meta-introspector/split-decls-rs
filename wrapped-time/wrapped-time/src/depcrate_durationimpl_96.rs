// Generated macro for impl_96 (impl)
macro_rules! Depcrate_durationimpl_96 {
() => {
// Module: crate::duration
// Provides: {"impl_96"}
// Dependencies: {}
# [cfg (feature = "std")] impl Sub < Duration > for SystemTime { type Output = Self ; # [inline] # [track_caller] fn sub (self , duration : Duration) -> Self :: Output { if duration . is_zero () { self } else if duration . is_positive () { self - duration . unsigned_abs () } else { debug_assert ! (duration . is_negative ()) ; self + duration . unsigned_abs () } } }
};
}
