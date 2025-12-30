// Generated macro for jamo_map (function)
macro_rules! Depcrate_jamo_short_namejamo_map {
() => {
// Module: crate::jamo_short_name
// Provides: {"jamo_map"}
// Dependencies: {}
fn jamo_map (dir : & Path) -> Result < BTreeMap < u32 , String > > { let jamo_map = ucd_parse :: parse_by_codepoint :: < _ , JamoShortName > (dir) ? ; let mut map = BTreeMap :: new () ; for (cp , jamo) in jamo_map { map . insert (cp . value () , jamo . name) ; } Ok (map) }
};
}
