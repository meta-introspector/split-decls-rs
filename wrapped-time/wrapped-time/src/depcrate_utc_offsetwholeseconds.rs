// Generated macro for WholeSeconds (type)
macro_rules! Depcrate_utc_offsetWholeSeconds {
() => {
// Module: crate::utc_offset
// Provides: {"WholeSeconds"}
// Dependencies: {}
# [doc = " The type capable of storing the range of whole seconds that a `UtcOffset` can encompass."] type WholeSeconds = RangedI32 < { Hours :: MIN . get () as i32 * Second :: per_t :: < i32 > (Hour) + Minutes :: MIN . get () as i32 * Second :: per_t :: < i32 > (Minute) + Seconds :: MIN . get () as i32 } , { Hours :: MAX . get () as i32 * Second :: per_t :: < i32 > (Hour) + Minutes :: MAX . get () as i32 * Second :: per_t :: < i32 > (Minute) + Seconds :: MAX . get () as i32 } , > ;
};
}
