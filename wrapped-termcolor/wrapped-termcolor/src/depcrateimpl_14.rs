// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl FromStr for ColorChoice { type Err = ColorChoiceParseError ; fn from_str (s : & str) -> Result < ColorChoice , ColorChoiceParseError > { match s . to_lowercase () . as_str () { "always" => Ok (ColorChoice :: Always) , "always-ansi" => Ok (ColorChoice :: AlwaysAnsi) , "never" => Ok (ColorChoice :: Never) , "auto" => Ok (ColorChoice :: Auto) , unknown => Err (ColorChoiceParseError { unknown_choice : unknown . to_string () , }) , } } }
};
}
