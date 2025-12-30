// Generated macro for grapheme_cluster (function)
macro_rules! Depcrate_brkgrapheme_cluster {
() => {
// Module: crate::brk
// Provides: {"grapheme_cluster"}
// Dependencies: {}
pub fn grapheme_cluster (args : ArgMatches < '_ >) -> Result < () > { let ucd_dir = args . ucd_dir () ? ; let vals : Vec < GraphemeClusterBreak > = ucd_parse :: parse (& ucd_dir) ? ; let mut byval : BTreeMap < String , BTreeSet < u32 > > = BTreeMap :: new () ; for x in & vals { byval . entry (x . value . clone ()) . or_insert (BTreeSet :: new ()) . extend (x . codepoints . into_iter () . map (| c | c . value ())) ; } let mut wtr = args . writer ("grapheme_cluster_break") ? ; if args . is_present ("enum") { wtr . ranges_to_enum (args . name () , & byval) ? ; } else { wtr . names (byval . keys ()) ? ; for (val , set) in byval { wtr . ranges (& val , & set) ? ; } } Ok (()) }
};
}
