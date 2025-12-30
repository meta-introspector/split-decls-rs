// Generated macro for Constants (struct)
macro_rules! Depcrate_validateConstants {
() => {
// Module: crate::validate
// Provides: {"Constants"}
// Dependencies: {}
# [doc = " Rational property-related constants for a specific float type."] # [allow (dead_code)] # [derive (Debug)] pub struct Constants { # [doc = " The minimum positive value (a subnormal)."] min_subnormal : BigRational , # [doc = " The maximum possible finite value."] max : BigRational , # [doc = " Cutoff between rounding to zero and rounding to the minimum value (min subnormal)."] zero_cutoff : BigRational , # [doc = " Cutoff between rounding to the max value and rounding to infinity."] inf_cutoff : BigRational , # [doc = " Opposite of `inf_cutoff`"] neg_inf_cutoff : BigRational , # [doc = " The powers of two for all relevant integers."] powers_of_two : BTreeMap < i32 , BigRational > , # [doc = " Half of each power of two. ULP = \"unit in last position\"."] # [doc = ""] # [doc = " This is a mapping from integers to half the precision available at that exponent. In other"] # [doc = " words, `0.5 * 2^n` = `2^(n-1)`, which is half the distance between `m * 2^n` and"] # [doc = " `(m + 1) * 2^n`, m ∈ ℤ."] # [doc = ""] # [doc = " So, this is the maximum error from a real number to its floating point representation,"] # [doc = " assuming the float type can represent the exponent."] half_ulp : BTreeMap < i32 , BigRational > , # [doc = " Handy to have around so we don't need to reallocate for it"] two : BigInt , }
};
}
