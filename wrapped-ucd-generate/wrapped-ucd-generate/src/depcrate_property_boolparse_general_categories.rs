// Generated macro for parse_general_categories (function)
macro_rules! Depcrate_property_boolparse_general_categories {
() => {
// Module: crate::property_bool
// Provides: {"parse_general_categories"}
// Dependencies: {}
fn parse_general_categories < P : AsRef < Path > > (ucd_dir : P ,) -> Result < BTreeMap < String , BTreeSet < u32 > > > { let propvals = PropertyValues :: from_ucd_dir (& ucd_dir) ? ; let unexpanded = ucd_parse :: parse (& ucd_dir) ? ; let rows : Vec < _ > = UnicodeDataExpander :: new (unexpanded) . collect () ; let mut bycat : BTreeMap < String , BTreeSet < u32 > > = BTreeMap :: new () ; for row in rows { let gc = propvals . canonical ("gc" , & row . general_category) ? . to_string () ; bycat . entry (gc) . or_insert (BTreeSet :: new ()) . insert (row . codepoint . value ()) ; } Ok (bycat) }
};
}
