// Generated macro for fill_bounded (function)
macro_rules! Depcrate_segmenterfill_bounded {
() => {
// Module: crate::segmenter
// Provides: {"fill_bounded"}
// Dependencies: {}
# [doc = " Fill `dst` at range `r` with `value`, ignoring any out of bounds ranges"] fn fill_bounded (dst : & mut [u8] , r : RangeInclusive < u32 > , value : u8) { let start = * r . start () as usize ; let end = cmp :: min (* r . end () as usize , dst . len () - 1) ; if start >= dst . len () { return ; } dst [start ..= end] . fill (value) ; }
};
}
