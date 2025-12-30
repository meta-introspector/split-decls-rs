// Generated macro for tests (module)
macro_rules! Depcrate_scriptstests {
() => {
// Module: crate::scripts
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Script ; # [test] fn parse_single () { let line = "10A7F         ; Old_South_Arabian # Po       OLD SOUTH ARABIAN NUMERIC INDICATOR\n" ; let row : Script = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x10A7F) ; assert_eq ! (row . script , "Old_South_Arabian") ; } # [test] fn parse_range () { let line = "1200..1248    ; Ethiopic # Lo  [73] ETHIOPIC SYLLABLE HA..ETHIOPIC SYLLABLE QWA\n" ; let row : Script = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x1200 , 0x1248)) ; assert_eq ! (row . script , "Ethiopic") ; } }
};
}
