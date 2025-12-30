// Generated macro for tests (module)
macro_rules! Depcrate_special_casingtests {
() => {
// Module: crate::special_casing
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: SpecialCaseMapping ; # [test] fn parse_no_conds () { let line = "1F52; 1F52; 03A5 0313 0300; 03A5 0313 0300; # GREEK SMALL LETTER UPSILON WITH PSILI AND VARIA\n" ; let row : SpecialCaseMapping = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x1F52) ; assert_eq ! (row . lowercase , vec ! [0x1F52]) ; assert_eq ! (row . titlecase , vec ! [0x03A5 , 0x0313 , 0x0300]) ; assert_eq ! (row . uppercase , vec ! [0x03A5 , 0x0313 , 0x0300]) ; assert ! (row . conditions . is_empty ()) ; } # [test] fn parse_conds () { let line = "0307; ; 0307; 0307; tr After_I; # COMBINING DOT ABOVE\n" ; let row : SpecialCaseMapping = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x0307) ; assert ! (row . lowercase . is_empty ()) ; assert_eq ! (row . titlecase , vec ! [0x0307]) ; assert_eq ! (row . uppercase , vec ! [0x0307]) ; assert_eq ! (row . conditions , vec ! ["tr" , "After_I"]) ; } }
};
}
