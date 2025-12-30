// Generated macro for tests (module)
macro_rules! Depcrate_case_foldingtests {
() => {
// Module: crate::case_folding
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { CaseFold , CaseStatus } ; # [test] fn parse_common () { let line = "0150; C; 0151; # LATIN CAPITAL LETTER O WITH DOUBLE ACUTE\n" ; let row : CaseFold = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x0150) ; assert_eq ! (row . status , CaseStatus :: Common) ; assert_eq ! (row . mapping , vec ! [0x0151]) ; } # [test] fn parse_full () { let line = "03B0; F; 03C5 0308 0301; # GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS\n" ; let row : CaseFold = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x03B0) ; assert_eq ! (row . status , CaseStatus :: Full) ; assert_eq ! (row . mapping , vec ! [0x03C5 , 0x0308 , 0x0301]) ; } # [test] fn parse_simple () { let line = "1F8F; S; 1F87; # GREEK CAPITAL LETTER ALPHA WITH DASIA AND PERISPOMENI AND PROSGEGRAMMENI\n" ; let row : CaseFold = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x1F8F) ; assert_eq ! (row . status , CaseStatus :: Simple) ; assert_eq ! (row . mapping , vec ! [0x1F87]) ; } # [test] fn parse_special () { let line = "0049; T; 0131; # LATIN CAPITAL LETTER I\n" ; let row : CaseFold = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x0049) ; assert_eq ! (row . status , CaseStatus :: Special) ; assert_eq ! (row . mapping , vec ! [0x0131]) ; } }
};
}
