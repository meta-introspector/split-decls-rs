// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_numeric_valuestests {
() => {
// Module: crate::extracted::derived_numeric_values
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedNumericValues ; # [test] fn parse_single () { let line = "0030          ; 0.0 ; ; 0 # Nd       DIGIT ZERO\n" ; let row : DerivedNumericValues = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x0030) ; assert_eq ! (row . numeric_value_decimal , "0.0") ; assert_eq ! (row . numeric_value_fraction , "0") ; } # [test] fn parse_range () { let line = "11FC9..11FCA  ; 0.0625 ; ; 1/16 # No   [2] TAMIL FRACTION ONE SIXTEENTH-1..TAMIL FRACTION ONE SIXTEENTH-2\n" ; let row : DerivedNumericValues = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x11FC9 , 0x11FCA)) ; assert_eq ! (row . numeric_value_decimal , "0.0625") ; assert_eq ! (row . numeric_value_fraction , "1/16") ; } }
};
}
