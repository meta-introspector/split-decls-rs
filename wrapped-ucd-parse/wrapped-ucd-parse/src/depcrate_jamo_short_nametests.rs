// Generated macro for tests (module)
macro_rules! Depcrate_jamo_short_nametests {
() => {
// Module: crate::jamo_short_name
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: JamoShortName ; # [test] fn parse1 () { let line = "1164; YAE # HANGUL JUNGSEONG YAE\n" ; let row : JamoShortName = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x1164) ; assert_eq ! (row . name , "YAE") ; } # [test] fn parse2 () { let line = "110B;     # HANGUL CHOSEONG IEUNG\n" ; let row : JamoShortName = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x110B) ; assert_eq ! (row . name , "") ; } }
};
}
