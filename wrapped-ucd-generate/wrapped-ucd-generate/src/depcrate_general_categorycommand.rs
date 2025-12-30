// Generated macro for command (function)
macro_rules! Depcrate_general_categorycommand {
() => {
// Module: crate::general_category
// Provides: {"command"}
// Dependencies: {}
pub fn command (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let propvals = PropertyValues :: from_ucd_dir (& dir) ? ; let filter = args . filter (| name | propvals . canonical ("gc" , name)) ? ; let unexpanded = ucd_parse :: parse (& dir) ? ; if args . is_present ("list-categories") { return print_property_values (& propvals , "General_Category") ; } let mut bycat = expand_into_categories (unexpanded , & propvals) ? ; if ! args . is_present ("enum") && ! args . is_present ("rust-enum") { for (name , set) in related (& propvals , & bycat) { if filter . contains (& name) { bycat . insert (name , set) ; } } } let bycat = bycat . into_iter () . filter (| & (ref name , _) | filter . contains (name)) . collect () ; let mut wtr = args . writer ("general_category") ? ; if args . is_present ("enum") { wtr . ranges_to_enum (args . name () , & bycat) ? ; } else if args . is_present ("rust-enum") { let variants = bycat . keys () . map (String :: as_str) . collect :: < Vec < _ > > () ; wtr . ranges_to_rust_enum (args . name () , & variants , & bycat) ? ; } else if args . is_present ("combined") { wtr . ranges_to_combined (args . name () , & bycat) ? ; } else { wtr . names (bycat . keys () . filter (| n | filter . contains (n))) ? ; for (name , set) in bycat { wtr . ranges (& name , & set) ? ; } } Ok (()) }
};
}
