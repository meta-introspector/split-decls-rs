// Generated macro for POWERS_OF_TWO_RANGE (const)
macro_rules! Depcrate_validatePOWERS_OF_TWO_RANGE {
() => {
// Module: crate::validate
// Provides: {"POWERS_OF_TWO_RANGE"}
// Dependencies: {}
# [doc = " Powers of two that we store for constants. Account for binary128 which has a 15-bit exponent."] const POWERS_OF_TWO_RANGE : RangeInclusive < i32 > = (- (2 << 15)) ..= (2 << 15) ;
};
}
