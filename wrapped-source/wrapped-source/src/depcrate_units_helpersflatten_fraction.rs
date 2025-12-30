// Generated macro for flatten_fraction (function)
macro_rules! Depcrate_units_helpersflatten_fraction {
() => {
// Module: crate::units::helpers
// Provides: {"flatten_fraction"}
// Dependencies: {}
# [doc = " Converts a fractional number into its byte representation for numerator and denominator, along with its sign."] pub (crate) fn flatten_fraction (fraction : IcuRatio) -> (Vec < u8 > , Vec < u8 > , Sign) { let fraction = fraction . get_ratio () ; let numer_bytes = fraction . numer () . to_bytes_le () . 1 ; let denom_bytes = fraction . denom () . to_bytes_le () . 1 ; let sign = match fraction . is_negative () { true => Sign :: Negative , false => Sign :: Positive , } ; (numer_bytes , denom_bytes , sign) }
};
}
