// Generated macro for Nanoseconds (type)
macro_rules! Depcrate_durationNanoseconds {
() => {
// Module: crate::duration
// Provides: {"Nanoseconds"}
// Dependencies: {}
# [doc = " The type of the `nanosecond` field of `Duration`."] type Nanoseconds = RangedI32 < { - Nanosecond :: per_t :: < i32 > (Second) + 1 } , { Nanosecond :: per_t :: < i32 > (Second) - 1 } > ;
};
}
