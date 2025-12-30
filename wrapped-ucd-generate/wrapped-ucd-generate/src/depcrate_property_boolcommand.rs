// Generated macro for command (function)
macro_rules! Depcrate_property_boolcommand {
() => {
// Module: crate::property_bool
// Provides: {"command"}
// Dependencies: {}
pub fn command (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let by_name = parse_properties (& dir) ? ; let properties = PropertyNames :: from_ucd_dir (& dir) ? ; let filter = args . filter (| name | properties . canonical (name)) ? ; if args . is_present ("list-properties") { for name in by_name . keys () { println ! ("{}" , name) ; } return Ok (()) ; } let mut wtr = args . writer ("prop_list") ? ; wtr . names (by_name . keys () . filter (| n | filter . contains (n))) ? ; for (name , set) in by_name { if filter . contains (& name) { wtr . ranges (& name , & set) ? ; } } Ok (()) }
};
}
