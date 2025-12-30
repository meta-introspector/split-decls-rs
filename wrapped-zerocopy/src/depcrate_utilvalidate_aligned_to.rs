// Generated macro for validate_aligned_to (function)
macro_rules! Depcrate_utilvalidate_aligned_to {
() => {
// Module: crate::util
// Provides: {"validate_aligned_to"}
// Dependencies: {}
# [doc = " Validates that `t` is aligned to `align_of::<U>()`."] # [inline (always)] pub (crate) fn validate_aligned_to < T : AsAddress , U > (t : T) -> Result < () , AlignmentError < () , U > > { # [allow (clippy :: arithmetic_side_effects)] let remainder = t . addr () % mem :: align_of :: < U > () ; if remainder == 0 { Ok (()) } else { Err (unsafe { AlignmentError :: new_unchecked (()) }) } }
};
}
