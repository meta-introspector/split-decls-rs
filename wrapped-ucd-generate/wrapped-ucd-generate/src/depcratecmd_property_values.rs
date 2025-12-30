// Generated macro for cmd_property_values (function)
macro_rules! Depcratecmd_property_values {
() => {
// Module: crate
// Provides: {"cmd_property_values"}
// Dependencies: {}
fn cmd_property_values (args : ArgMatches < '_ >) -> Result < () > { use crate :: util :: { PropertyNames , PropertyValues } ; use std :: collections :: BTreeMap ; let dir = args . ucd_dir () ? ; let values = PropertyValues :: from_ucd_dir (& dir) ? ; let names = PropertyNames :: from_ucd_dir (& dir) ? ; let filter = args . filter (| name | names . canonical (name)) ? ; let mut actual_values = BTreeMap :: new () ; for (k , v) in & values . value { if filter . contains (k) { actual_values . insert (k . to_string () , v . clone ()) ; } } let mut wtr = args . writer ("property_values") ? ; wtr . string_to_string_to_string (args . name () , & actual_values) ? ; Ok (()) }
};
}
