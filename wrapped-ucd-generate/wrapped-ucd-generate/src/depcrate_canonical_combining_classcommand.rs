// Generated macro for command (function)
macro_rules! Depcrate_canonical_combining_classcommand {
() => {
// Module: crate::canonical_combining_class
// Provides: {"command"}
// Dependencies: {}
pub fn command (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let propvals = PropertyValues :: from_ucd_dir (& dir) ? ; let rows : Vec < UnicodeData > = ucd_parse :: parse (& dir) ? ; let ccc_name = | ccc : u8 | { propvals . canonical ("canonicalcombiningclass" , & ccc . to_string ()) } ; if args . is_present ("list-classes") { return print_property_values (& propvals , "Canonical_Combining_Class") ; } let mut name_map : BTreeMap < isize , String > = BTreeMap :: new () ; let mut by_name : BTreeMap < String , BTreeSet < u32 > > = BTreeMap :: new () ; let mut assigned = BTreeSet :: new () ; for row in rows { assigned . insert (row . codepoint . value ()) ; let ccc_value = row . canonical_combining_class ; let ccc_name = ccc_name (ccc_value) ? ; name_map . entry (ccc_value as isize) . or_insert_with (| | ccc_name . clone ()) ; by_name . entry (ccc_name) . or_insert (BTreeSet :: new ()) . insert (row . codepoint . value ()) ; } let not_reordered_name = ccc_name (0) ? ; for cp in 0 ..= 0x10FFFF { if ! assigned . contains (& cp) { by_name . get_mut (& not_reordered_name) . unwrap () . insert (cp) ; } } let mut wtr = args . writer ("canonical_combining_class") ? ; if args . is_present ("enum") { wtr . ranges_to_enum (args . name () , & by_name) ? ; } else if args . is_present ("rust-enum") { wtr . ranges_to_rust_enum_with_custom_discriminants (args . name () , & name_map , & by_name ,) ? ; } else { wtr . names (by_name . keys ()) ? ; for (name , set) in by_name { wtr . ranges (& name , & set) ? ; } } Ok (()) }
};
}
