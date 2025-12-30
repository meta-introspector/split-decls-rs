// Generated macro for Minutes (type)
macro_rules! Depcrate_utc_offsetMinutes {
() => {
// Module: crate::utc_offset
// Provides: {"Minutes"}
// Dependencies: {}
# [doc = " The type of the `minutes` field of `UtcOffset`."] type Minutes = RangedI8 < { - (Minute :: per_t :: < i8 > (Hour) - 1) } , { Minute :: per_t :: < i8 > (Hour) - 1 } > ;
};
}
