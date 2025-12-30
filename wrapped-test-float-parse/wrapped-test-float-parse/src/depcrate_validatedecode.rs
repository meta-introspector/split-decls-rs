// Generated macro for decode (function)
macro_rules! Depcrate_validatedecode {
() => {
// Module: crate::validate
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decompose a float into its integral components. This includes the implicit bit."] # [doc = ""] # [doc = " If `allow_nan` is `false`, panic if `NaN` values are reached."] fn decode < F : Float > (f : F) -> FloatRes < F > { let ione = F :: SInt :: ONE ; let izero = F :: SInt :: ZERO ; let mut exponent_biased = f . exponent () ; let mut mantissa = f . mantissa () . to_signed () ; if exponent_biased == 0 { if mantissa == izero { return FloatRes :: Zero ; } exponent_biased += 1 ; } else if exponent_biased == F :: EXP_SAT { if mantissa != izero { return FloatRes :: Nan ; } if f . is_sign_negative () { return FloatRes :: NegInf ; } return FloatRes :: Inf ; } else { mantissa |= ione << F :: MAN_BITS ; } let mut exponent = i32 :: try_from (exponent_biased) . unwrap () ; exponent -= i32 :: try_from (F :: EXP_BIAS + F :: MAN_BITS) . unwrap () ; if f . is_sign_negative () { mantissa = mantissa . wrapping_neg () ; } FloatRes :: Real { sig : mantissa , exp : exponent } }
};
}
