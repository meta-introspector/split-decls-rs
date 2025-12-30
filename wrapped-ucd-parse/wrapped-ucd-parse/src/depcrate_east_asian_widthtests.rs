// Generated macro for tests (module)
macro_rules! Depcrate_east_asian_widthtests {
() => {
// Module: crate::east_asian_width
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: EastAsianWidth ; # [test] fn parse_single () { let line = "27E7;Na          # Pe         MATHEMATICAL RIGHT WHITE SQUARE BRACKET\n" ; let row : EastAsianWidth = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x27E7) ; assert_eq ! (row . width , "Na") ; } # [test] fn parse_range () { let line = "1F57B..1F594;N   # So    [26] LEFT HAND TELEPHONE RECEIVER..REVERSED VICTORY HAND\n" ; let row : EastAsianWidth = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x1F57B , 0x1F594)) ; assert_eq ! (row . width , "N") ; } }
};
}
