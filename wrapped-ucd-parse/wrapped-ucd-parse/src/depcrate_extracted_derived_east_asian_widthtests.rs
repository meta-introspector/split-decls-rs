// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_east_asian_widthtests {
() => {
// Module: crate::extracted::derived_east_asian_width
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedEastAsianWidth ; # [test] fn parse_single () { let line = "00A0          ; N # Zs       NO-BREAK SPACE\n" ; let row : DerivedEastAsianWidth = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x00A0) ; assert_eq ! (row . east_asian_width , "N") ; } # [test] fn parse_range () { let line = "FF10..FF19    ; F # Nd  [10] FULLWIDTH DIGIT ZERO..FULLWIDTH DIGIT NINE\n" ; let row : DerivedEastAsianWidth = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0xFF10 , 0xFF19)) ; assert_eq ! (row . east_asian_width , "F") ; } }
};
}
