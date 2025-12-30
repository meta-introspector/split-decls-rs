// Generated macro for expand_into_categories (function)
macro_rules! Depcrate_general_categoryexpand_into_categories {
() => {
// Module: crate::general_category
// Provides: {"expand_into_categories"}
// Dependencies: {}
# [doc = " Expand a list of UnicodeData rows and group by category."] pub fn expand_into_categories (unexpanded : Vec < UnicodeData > , propvals : & PropertyValues ,) -> Result < BTreeMap < String , BTreeSet < u32 > > > { let rows : Vec < _ > = UnicodeDataExpander :: new (unexpanded) . collect () ; let mut bycat : BTreeMap < String , BTreeSet < u32 > > = BTreeMap :: new () ; let mut assigned = BTreeSet :: new () ; for row in rows { assigned . insert (row . codepoint . value ()) ; let gc = propvals . canonical ("gc" , & row . general_category) ? . to_string () ; bycat . entry (gc) . or_insert (BTreeSet :: new ()) . insert (row . codepoint . value ()) ; } let unassigned_name = propvals . canonical ("gc" , "unassigned") ? . to_string () ; bycat . insert (unassigned_name . clone () , BTreeSet :: new ()) ; for cp in 0 ..= 0x10FFFF { if ! assigned . contains (& cp) { bycat . get_mut (& unassigned_name) . unwrap () . insert (cp) ; } } Ok (bycat) }
};
}
