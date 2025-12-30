// Generated macro for tests (module)
macro_rules! Depcrate_derived_normalization_propertiestests {
() => {
// Module: crate::derived_normalization_properties
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedNormalizationProperty ; # [test] fn parse_single () { let line = "00A0          ; Changes_When_NFKC_Casefolded # Zs       NO-BREAK SPACE\n" ; let row : DerivedNormalizationProperty = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0xA0) ; assert_eq ! (row . property , "Changes_When_NFKC_Casefolded") ; } # [test] fn parse_range () { let line = "0041..005A    ; Changes_When_NFKC_Casefolded # L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z\n" ; let row : DerivedNormalizationProperty = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x41 , 0x5A)) ; assert_eq ! (row . property , "Changes_When_NFKC_Casefolded") ; } }
};
}
