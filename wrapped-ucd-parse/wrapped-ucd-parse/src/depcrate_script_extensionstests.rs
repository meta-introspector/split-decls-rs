// Generated macro for tests (module)
macro_rules! Depcrate_script_extensionstests {
() => {
// Module: crate::script_extensions
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: ScriptExtension ; # [test] fn parse_single () { let line = "060C          ; Arab Syrc Thaa # Po       ARABIC COMMA\n" ; let row : ScriptExtension = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x060C) ; assert_eq ! (row . scripts , vec ! ["Arab" , "Syrc" , "Thaa"]) ; } # [test] fn parse_range () { let line = "A836..A837    ; Deva Gujr Guru Kthi Mahj Modi Sind Takr Tirh # So   [2] NORTH INDIC QUARTER MARK..NORTH INDIC PLACEHOLDER MARK\n" ; let row : ScriptExtension = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0xA836 , 0xA837)) ; assert_eq ! (row . scripts , vec ! ["Deva" , "Gujr" , "Guru" , "Kthi" , "Mahj" , "Modi" , "Sind" , "Takr" , "Tirh" ,]) ; } }
};
}
