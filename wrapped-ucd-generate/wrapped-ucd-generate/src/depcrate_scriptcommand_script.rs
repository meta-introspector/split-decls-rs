// Generated macro for command_script (function)
macro_rules! Depcrate_scriptcommand_script {
() => {
// Module: crate::script
// Provides: {"command_script"}
// Dependencies: {}
pub fn command_script (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let propvals = PropertyValues :: from_ucd_dir (& dir) ? ; let filter = args . filter (| name | propvals . canonical ("Script" , name)) ? ; if args . is_present ("list-scripts") { return print_property_values (& propvals , "Script") ; } let mut by_name : BTreeMap < String , BTreeSet < u32 > > = BTreeMap :: new () ; let scripts : Vec < Script > = ucd_parse :: parse (& dir) ? ; for x in & scripts { by_name . entry (x . script . clone ()) . or_insert (BTreeSet :: new ()) . extend (x . codepoints . into_iter () . map (| c | c . value ())) ; } let mut wtr = args . writer ("script") ? ; if args . is_present ("enum") { wtr . ranges_to_enum (args . name () , & by_name) ? ; } else if args . is_present ("rust-enum") { let mut variants = vec ! ["Unknown"] ; variants . extend (by_name . keys () . map (String :: as_str)) ; wtr . ranges_to_rust_enum (args . name () , & variants , & by_name) ? ; } else if args . is_present ("combined") { wtr . ranges_to_combined (args . name () , & by_name) ? ; } else { wtr . names (by_name . keys () . filter (| n | filter . contains (n))) ? ; for (name , set) in by_name { if filter . contains (& name) { wtr . ranges (& name , & set) ? ; } } } Ok (()) }
};
}
