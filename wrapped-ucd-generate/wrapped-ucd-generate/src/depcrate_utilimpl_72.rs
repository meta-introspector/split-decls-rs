// Generated macro for impl_72 (impl)
macro_rules! Depcrate_utilimpl_72 {
() => {
// Module: crate::util
// Provides: {"impl_72"}
// Dependencies: {}
impl PropertyNames { pub fn from_ucd_dir < P : AsRef < Path > > (ucd_dir : P) -> Result < PropertyNames > { use ucd_parse :: UcdFile ; let make_key = | mut value | { ucd_util :: symbolic_name_normalize (& mut value) ; value } ; let mut map = BTreeMap :: new () ; for result in PropertyAlias :: from_dir (ucd_dir) ? { let a = result ? ; let canon = a . long . to_string () ; for alias in a . aliases { map . insert (make_key (alias) , canon . clone ()) ; } map . insert (make_key (a . abbreviation) , canon . clone ()) ; map . insert (make_key (a . long) , canon) ; } const EMOJI_PROPERTY_NAMES : & 'static [& 'static str] = & ["Emoji" , "Emoji_Presentation" , "Emoji_Modifier" , "Emoji_Modifier_Base" , "Emoji_Component" , "Extended_Pictographic" ,] ; for name in EMOJI_PROPERTY_NAMES { map . insert (make_key (name . to_string ()) , name . to_string ()) ; } Ok (PropertyNames (map)) } # [doc = " Return the \"canonical\" or \"long\" property name for the given property"] # [doc = " name. If no such property exists, return an error."] pub fn canonical (& self , key : & str) -> Result < String > { let mut key = key . to_string () ; ucd_util :: symbolic_name_normalize (& mut key) ; match self . 0 . get (& key) . map (| v | & * * v) { Some (v) => Ok (v . to_string ()) , None => err ! ("unrecognized property: {:?}" , key) , } } }
};
}
