// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_combining_classtests {
() => {
// Module: crate::extracted::derived_combining_class
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedCombiningClass ; # [test] fn parse_single () { let line = "0020          ; 0 # Zs       SPACE\n" ; let row : DerivedCombiningClass = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x0020) ; assert_eq ! (row . combining_class , "0") ; } # [test] fn parse_range () { let line = "1DD1..1DF5    ; 230 # Mn  [37] COMBINING UR ABOVE..COMBINING UP TACK ABOVE\n" ; let row : DerivedCombiningClass = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x1DD1 , 0x1DF5)) ; assert_eq ! (row . combining_class , "230") ; } }
};
}
