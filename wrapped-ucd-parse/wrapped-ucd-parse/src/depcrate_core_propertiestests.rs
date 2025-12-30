// Generated macro for tests (module)
macro_rules! Depcrate_core_propertiestests {
() => {
// Module: crate::core_properties
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: CoreProperty ; # [test] fn parse_single () { let line = "1163D         ; Case_Ignorable # Mn       MODI SIGN ANUSVARA\n" ; let row : CoreProperty = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x1163D) ; assert_eq ! (row . property , "Case_Ignorable") ; } # [test] fn parse_range () { let line = "11133..11134  ; Grapheme_Link # Mn   [2] CHAKMA VIRAMA..CHAKMA MAAYYAA\n" ; let row : CoreProperty = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x11133 , 0x11134)) ; assert_eq ! (row . property , "Grapheme_Link") ; } }
};
}
