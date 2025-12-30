// Generated macro for Seconds (type)
macro_rules! Depcrate_utc_offsetSeconds {
() => {
// Module: crate::utc_offset
// Provides: {"Seconds"}
// Dependencies: {}
# [doc = " The type of the `seconds` field of `UtcOffset`."] type Seconds = RangedI8 < { - (Second :: per_t :: < i8 > (Minute) - 1) } , { Second :: per_t :: < i8 > (Minute) - 1 } > ;
};
}
