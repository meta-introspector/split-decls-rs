// Generated macro for is_scientific_number (function)
macro_rules! Depcrate_units_helpersis_scientific_number {
() => {
// Module: crate::units::helpers
// Provides: {"is_scientific_number"}
// Dependencies: {}
# [doc = " Checks if a string is a valid scientific notation number."] # [doc = " Returns true if the string is a valid scientific notation number, false otherwise.  "] pub (crate) fn is_scientific_number (s : & str) -> bool { let mut parts = s . split ('E') ; let base = parts . next () . unwrap_or ("0") ; let exponent = parts . next () . unwrap_or ("0") ; if parts . next () . is_some () { return false ; } ! contains_alphabetic_chars (base) && ! contains_alphabetic_chars (exponent) }
};
}
