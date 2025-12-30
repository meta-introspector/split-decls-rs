// Generated macro for command_script_extension (function)
macro_rules! Depcrate_scriptcommand_script_extension {
() => {
// Module: crate::script
// Provides: {"command_script_extension"}
// Dependencies: {}
pub fn command_script_extension (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let propvals = PropertyValues :: from_ucd_dir (& dir) ? ; let filter = args . filter (| name | propvals . canonical ("Script" , name)) ? ; if args . is_present ("list-script-extensions") { return print_property_values (& propvals , "Script") ; } let mut by_name : BTreeMap < String , BTreeSet < u32 > > = BTreeMap :: new () ; let mut seen : BTreeSet < u32 > = BTreeSet :: new () ; let exts : Vec < ScriptExtension > = ucd_parse :: parse (& dir) ? ; for x in & exts { seen . extend (x . codepoints . into_iter () . map (| c | c . value ())) ; for name in & x . scripts { let name = propvals . canonical ("Script" , name) ? ; by_name . entry (name) . or_insert (BTreeSet :: new ()) . extend (x . codepoints . into_iter () . map (| c | c . value ())) ; } } let scripts : Vec < Script > = ucd_parse :: parse (& dir) ? ; for x in & scripts { if ! by_name . contains_key (& x . script) { by_name . insert (x . script . clone () , BTreeSet :: new ()) ; } for cp in x . codepoints . into_iter () . map (| c | c . value ()) { if ! seen . contains (& cp) { by_name . get_mut (& x . script) . unwrap () . insert (cp) ; } } } let mut wtr = args . writer ("script_extension") ? ; wtr . names (by_name . keys () . filter (| n | filter . contains (n))) ? ; for (name , set) in by_name { if filter . contains (& name) { wtr . ranges (& name , & set) ? ; } } Ok (()) }
};
}
