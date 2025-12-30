// Generated macro for cmd_property_names (function)
macro_rules! Depcratecmd_property_names {
() => {
// Module: crate
// Provides: {"cmd_property_names"}
// Dependencies: {}
fn cmd_property_names (args : ArgMatches < '_ >) -> Result < () > { use crate :: util :: PropertyNames ; use std :: collections :: BTreeMap ; let dir = args . ucd_dir () ? ; let names = PropertyNames :: from_ucd_dir (& dir) ? ; let filter = args . filter (| name | names . canonical (name)) ? ; let mut actual_names = BTreeMap :: new () ; for (k , v) in & names . 0 { if filter . contains (v) { actual_names . insert (k . to_string () , v . to_string ()) ; } } let mut wtr = args . writer ("property_names") ? ; wtr . string_to_string (args . name () , & actual_names) ? ; Ok (()) }
};
}
