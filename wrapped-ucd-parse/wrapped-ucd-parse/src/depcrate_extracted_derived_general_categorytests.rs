// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_general_categorytests {
() => {
// Module: crate::extracted::derived_general_category
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedGeneralCategory ; # [test] fn parse_single () { let line = "04D9          ; Ll #       CYRILLIC SMALL LETTER SCHWA\n" ; let row : DerivedGeneralCategory = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x04D9) ; assert_eq ! (row . general_category , "Ll") ; } # [test] fn parse_range () { let line = "0660..0669    ; Nd #  [10] ARABIC-INDIC DIGIT ZERO..ARABIC-INDIC DIGIT NINE" ; let row : DerivedGeneralCategory = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x0660 , 0x0669)) ; assert_eq ! (row . general_category , "Nd") ; } }
};
}
