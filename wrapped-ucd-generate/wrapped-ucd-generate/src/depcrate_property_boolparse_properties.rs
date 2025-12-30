// Generated macro for parse_properties (function)
macro_rules! Depcrate_property_boolparse_properties {
() => {
// Module: crate::property_bool
// Provides: {"parse_properties"}
// Dependencies: {}
fn parse_properties < P : AsRef < Path > > (ucd_dir : P ,) -> Result < BTreeMap < String , BTreeSet < u32 > > > { let mut by_name : BTreeMap < String , BTreeSet < u32 > > = BTreeMap :: new () ; let prop_list : Vec < Property > = ucd_parse :: parse (& ucd_dir) ? ; for x in & prop_list { by_name . entry (x . property . clone ()) . or_insert (BTreeSet :: new ()) . extend (x . codepoints . into_iter () . map (| c | c . value ())) ; } let core_prop : Vec < CoreProperty > = ucd_parse :: parse (& ucd_dir) ? ; for x in & core_prop { by_name . entry (x . property . clone ()) . or_insert (BTreeSet :: new ()) . extend (x . codepoints . into_iter () . map (| c | c . value ())) ; } let unicode_data : Vec < UnicodeData > = ucd_parse :: parse (& ucd_dir) ? ; let bidi_mirrored = unicode_data . iter () . fold (BTreeSet :: new () , | mut set , x | { if x . bidi_mirrored { set . extend (x . codepoints () . into_iter () . map (| c | c . value ())) } set }) ; by_name . insert ("Bidi_Mirrored" . to_string () , bidi_mirrored) ; let emoji_prop : Vec < EmojiProperty > = match ucd_parse :: parse (& ucd_dir) { Ok (props) => props , Err (err) => match * err . kind () { ucd_parse :: ErrorKind :: Io (_) => { eprintln ! ("{}. skipping emoji properties. \
                     emoji-data.txt is included in UCD 13.0.0 and newer, and \
                     can be downloaded from https://unicode.org/Public/emoji/ \
                     for older releases." , err ,) ; vec ! [] } _ => return Err (From :: from (err)) , } , } ; for x in & emoji_prop { by_name . entry (x . property . clone ()) . or_insert (BTreeSet :: new ()) . extend (x . codepoints . into_iter () . map (| c | c . value ())) ; } Ok (by_name) }
};
}
