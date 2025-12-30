// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_nametests {
() => {
// Module: crate::extracted::derived_name
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedName ; # [test] fn parse_single () { let line = "0021          ; EXCLAMATION MARK\n" ; let row : DerivedName = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x0021) ; assert_eq ! (row . name , "EXCLAMATION MARK") ; } # [test] fn parse_range () { let line = "3400..4DBF    ; CJK UNIFIED IDEOGRAPH-*\n" ; let row : DerivedName = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x3400 , 0x4DBF)) ; assert_eq ! (row . name , "CJK UNIFIED IDEOGRAPH-*") ; } }
};
}
