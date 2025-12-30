// Generated macro for tests (module)
macro_rules! Depcrate_prop_listtests {
() => {
// Module: crate::prop_list
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Property ; # [test] fn parse_single () { let line = "061C          ; Bidi_Control # Cf       ARABIC LETTER MARK\n" ; let row : Property = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x061C) ; assert_eq ! (row . property , "Bidi_Control") ; } # [test] fn parse_range () { let line = "0009..000D    ; White_Space # Cc   [5] <control-0009>..<control-000D>\n" ; let row : Property = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x0009 , 0x000D)) ; assert_eq ! (row . property , "White_Space") ; } }
};
}
