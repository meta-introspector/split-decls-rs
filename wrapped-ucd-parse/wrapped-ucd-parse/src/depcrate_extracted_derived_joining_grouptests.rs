// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_joining_grouptests {
() => {
// Module: crate::extracted::derived_joining_group
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedJoiningGroup ; # [test] fn parse_single () { let line = "0710          ; Alaph # Lo       SYRIAC LETTER ALAPH\n" ; let row : DerivedJoiningGroup = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x0710) ; assert_eq ! (row . joining_group , "Alaph") ; } # [test] fn parse_range () { let line = "0633..0634    ; Seen # Lo   [2] ARABIC LETTER SEEN..ARABIC LETTER SHEEN\n" ; let row : DerivedJoiningGroup = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x0633 , 0x0634)) ; assert_eq ! (row . joining_group , "Seen") ; } }
};
}
