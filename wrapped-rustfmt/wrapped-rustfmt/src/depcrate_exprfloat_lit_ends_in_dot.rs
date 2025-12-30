// Generated macro for float_lit_ends_in_dot (function)
macro_rules! Depcrate_exprfloat_lit_ends_in_dot {
() => {
// Module: crate::expr
// Provides: {"float_lit_ends_in_dot"}
// Dependencies: {}
pub (crate) fn float_lit_ends_in_dot (symbol : & str , suffix : Option < & str > , float_literal_trailing_zero : FloatLiteralTrailingZero ,) -> bool { match float_literal_trailing_zero { FloatLiteralTrailingZero :: Preserve => symbol . ends_with ('.') && suffix . is_none () , FloatLiteralTrailingZero :: IfNoPostfix | FloatLiteralTrailingZero :: Always => false , FloatLiteralTrailingZero :: Never => { let float_parts = parse_float_symbol (symbol) . unwrap () ; let has_postfix = float_parts . exponent . is_some () || suffix . is_some () ; let fractional_part_zero = float_parts . is_fractional_part_zero () ; ! has_postfix && fractional_part_zero } } }
};
}
