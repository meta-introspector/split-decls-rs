// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_line_breaktests {
() => {
// Module: crate::extracted::derived_line_break
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedLineBreak ; # [test] fn parse_single () { let line = "0028          ; OP # Ps       LEFT PARENTHESIS\n" ; let row : DerivedLineBreak = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x0028) ; assert_eq ! (row . line_break , "OP") ; } # [test] fn parse_range () { let line = "0030..0039    ; NU # Nd  [10] DIGIT ZERO..DIGIT NINE\n" ; let row : DerivedLineBreak = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x0030 , 0x0039)) ; assert_eq ! (row . line_break , "NU") ; } }
};
}
