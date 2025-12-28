macro_rules! deps {
    () => {
        AlignmentError!();
        AsAddress!();
    };
}

macro_rules! validate_aligned_to {
    () => {
        deps!();
        # [doc = " Validates that `t` is aligned to `align_of::<U>()`."] # [inline (always)] pub (crate) fn validate_aligned_to < T : AsAddress , U > (t : T) -> Result < () , AlignmentError < () , U > > { # [allow (clippy :: arithmetic_side_effects)] let remainder = t . addr () % mem :: align_of :: < U > () ; if remainder == 0 { Ok (()) } else { Err (unsafe { AlignmentError :: new_unchecked (()) }) } }
    };
}

validate_aligned_to!()