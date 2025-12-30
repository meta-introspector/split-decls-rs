// Generated macro for tests (module)
macro_rules! Depcrate_extracted_derived_joining_typetests {
() => {
// Module: crate::extracted::derived_joining_type
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DerivedJoiningType ; # [test] fn parse_single () { let line = "0628          ; D # Lo       ARABIC LETTER BEH\n" ; let row : DerivedJoiningType = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x0628) ; assert_eq ! (row . joining_type , "D") ; } # [test] fn parse_range () { let line = "1133B..1133C  ; T # Mn   [2] COMBINING BINDU BELOW..GRANTHA SIGN NUKTA\n" ; let row : DerivedJoiningType = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x1133B , 0x1133C)) ; assert_eq ! (row . joining_type , "T") ; } }
};
}
