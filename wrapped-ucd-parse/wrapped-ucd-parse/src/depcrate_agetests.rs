// Generated macro for tests (module)
macro_rules! Depcrate_agetests {
() => {
// Module: crate::age
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Age ; # [test] fn parse_single () { let line = "2BD2          ; 10.0 #       GROUP MARK\n" ; let row : Age = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x2BD2) ; assert_eq ! (row . age , "10.0") ; } # [test] fn parse_range () { let line = "11D0B..11D36  ; 10.0 #  [44] MASARAM GONDI LETTER AU..MASARAM GONDI VOWEL SIGN VOCALIC R\n" ; let row : Age = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x11D0B , 0x11D36)) ; assert_eq ! (row . age , "10.0") ; } }
};
}
