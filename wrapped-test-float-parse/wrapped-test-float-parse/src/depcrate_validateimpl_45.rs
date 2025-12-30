// Generated macro for impl_45 (impl)
macro_rules! Depcrate_validateimpl_45 {
() => {
// Module: crate::validate
// Provides: {"impl_45"}
// Dependencies: {}
impl Constants { pub fn new < F : Float > () -> Self { let two_int = & BigInt :: from_u32 (2) . unwrap () ; let two = & BigRational :: from_integer (2 . into ()) ; let min_subnormal = two . pow (- (F :: EXP_BIAS + F :: MAN_BITS - 1) . to_signed ()) ; let max = (two - two . pow (- F :: MAN_BITS . to_signed ())) * (two . pow (F :: EXP_BIAS . to_signed ())) ; let zero_cutoff = & min_subnormal / two_int ; let inf_cutoff = & max + two_int . pow (F :: EXP_BIAS - F :: MAN_BITS - 1) ; let neg_inf_cutoff = - & inf_cutoff ; let powers_of_two : BTreeMap < i32 , _ > = (POWERS_OF_TWO_RANGE) . map (| n | (n , two . pow (n))) . collect () ; let mut half_ulp = powers_of_two . clone () ; half_ulp . iter_mut () . for_each (| (_k , v) | * v = & * v / two_int) ; Self { min_subnormal , max , zero_cutoff , inf_cutoff , neg_inf_cutoff , powers_of_two , half_ulp , two : two_int . clone () , } } }
};
}
