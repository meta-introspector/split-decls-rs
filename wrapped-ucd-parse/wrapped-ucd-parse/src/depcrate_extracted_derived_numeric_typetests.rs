// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_numeric_typetests {
() => {
// Module: crate::extracted::derived_numeric_type
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedNumericType ; # [test] fn parse_single () { let line = "2189          ; Numeric # No       VULGAR FRACTION ZERO THIRDS\n" ; let row : DerivedNumericType = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x2189) ; assert_eq ! (row . numeric_type , "Numeric") ; } # [test] fn parse_range () { let line = "00B2..00B3    ; Digit # No   [2] SUPERSCRIPT TWO..SUPERSCRIPT THREE\n" ; let row : DerivedNumericType = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x00B2 , 0x00B3)) ; assert_eq ! (row . numeric_type , "Digit") ; } }
};
}
