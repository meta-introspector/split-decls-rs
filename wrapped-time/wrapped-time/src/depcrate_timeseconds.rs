// Generated macro for Seconds (type)
macro_rules! Depcrate_timeSeconds {
() => {
// Module: crate::time
// Provides: {"Seconds"}
// Dependencies: {}
# [doc = " The type of the `second` field of `Time`."] type Seconds = RangedU8 < 0 , { Second :: per_t :: < u8 > (Minute) - 1 } > ;
};
}
