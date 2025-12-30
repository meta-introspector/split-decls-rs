// Generated macro for impl_241 (impl)
macro_rules! Depcrate_jamo_short_nameimpl_241 {
() => {
// Module: crate::jamo_short_name
// Provides: {"impl_241"}
// Dependencies: {}
impl std :: str :: FromStr for JamoShortName { type Err = Error ; fn from_str (line : & str) -> Result < JamoShortName , Error > { let re_parts = regex ! (r"(?x)
                ^
                (?P<codepoint>[A-Z0-9]+);
                \s*
                (?P<name>[A-Z]*)
                " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid Jamo_Short_name line") , } ; Ok (JamoShortName { codepoint : caps ["codepoint"] . parse () ? , name : caps . name ("name") . unwrap () . as_str () . to_string () , }) } }
};
}
