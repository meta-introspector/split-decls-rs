// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_bidi_classtests {
() => {
// Module: crate::extracted::derived_bidi_class
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedBidiClass ; # [test] fn parse_single () { let line = "00B5          ; L # L&       MICRO SIGN\n" ; let row : DerivedBidiClass = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x00B5) ; assert_eq ! (row . bidi_class , "L") ; } # [test] fn parse_range () { let line = "0030..0039    ; EN # Nd  [10] DIGIT ZERO..DIGIT NINE\n" ; let row : DerivedBidiClass = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x0030 , 0x0039)) ; assert_eq ! (row . bidi_class , "EN") ; } }
};
}
