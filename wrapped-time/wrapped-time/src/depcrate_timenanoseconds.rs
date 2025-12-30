// Generated macro for Nanoseconds (type)
macro_rules! Depcrate_timeNanoseconds {
() => {
// Module: crate::time
// Provides: {"Nanoseconds"}
// Dependencies: {}
# [doc = " The type of the `nanosecond` field of `Time`."] type Nanoseconds = RangedU32 < 0 , { Nanosecond :: per_t :: < u32 > (Second) - 1 } > ;
};
}
