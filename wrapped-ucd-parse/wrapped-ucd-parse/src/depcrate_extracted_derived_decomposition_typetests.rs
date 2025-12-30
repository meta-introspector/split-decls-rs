// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_decomposition_typetests {
() => {
// Module: crate::extracted::derived_decomposition_type
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedDecompositionType ; # [test] fn parse_single () { let line = "00A0          ; Nobreak # Zs       NO-BREAK SPACE\n" ; let row : DerivedDecompositionType = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x00A0) ; assert_eq ! (row . decomposition_type , "Nobreak") ; } # [test] fn parse_range () { let line = "3070..3071    ; Canonical # Lo   [2] HIRAGANA LETTER BA..HIRAGANA LETTER PA\n" ; let row : DerivedDecompositionType = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x3070 , 0x3071)) ; assert_eq ! (row . decomposition_type , "Canonical") ; } }
};
}
