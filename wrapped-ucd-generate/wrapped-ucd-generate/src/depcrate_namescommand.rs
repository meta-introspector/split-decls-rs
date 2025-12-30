// Generated macro for command (function)
macro_rules! Depcrate_namescommand {
() => {
// Module: crate::names
// Provides: {"command"}
// Dependencies: {}
pub fn command (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let jamo_short_name_map = crate :: jamo_short_name :: table (Path :: new (dir)) ? ; let data = ucd_parse :: parse_by_codepoint (& dir) ? ; let aliases = if args . is_present ("no-aliases") { None } else { Some (ucd_parse :: parse_many_by_codepoint (& dir) ?) } ; let mut names = names_to_codepoint (& data , & aliases , & crate :: jamo_short_name :: table_ref (& jamo_short_name_map) , ! args . is_present ("no-ideograph") , ! args . is_present ("no-hangul") ,) ; if args . is_present ("normalize") { names = names . into_iter () . map (| (mut name , tagged) | { ucd_util :: character_name_normalize (& mut name) ; (name , tagged) }) . collect () ; } let mut wtr = args . writer ("names") ? ; if args . is_present ("tagged") { let mut map = BTreeMap :: new () ; for (name , (tag , cp)) in names { map . insert (name , tag . with_codepoint (cp)) ; } wtr . string_to_u64 (args . name () , & map) ? ; } else { let mut map = BTreeMap :: new () ; for (name , (_ , cp)) in names { map . insert (name , cp) ; } wtr . string_to_codepoint (args . name () , & map) ? ; } Ok (()) }
};
}
