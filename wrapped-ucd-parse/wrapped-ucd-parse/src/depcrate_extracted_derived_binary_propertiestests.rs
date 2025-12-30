// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_binary_propertiestests {
() => {
// Module: crate::extracted::derived_binary_properties
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedBinaryProperties ; # [test] fn parse_single () { let line = "0028          ; Bidi_Mirrored # Ps       LEFT PARENTHESIS\n" ; let row : DerivedBinaryProperties = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x0028) ; assert_eq ! (row . property , "Bidi_Mirrored") ; } # [test] fn parse_range () { let line = "2A3C..2A3E    ; Bidi_Mirrored # Sm   [3] INTERIOR PRODUCT..Z NOTATION RELATIONAL COMPOSITION\n" ; let row : DerivedBinaryProperties = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x2A3C , 0x2A3E)) ; assert_eq ! (row . property , "Bidi_Mirrored") ; } }
};
}
