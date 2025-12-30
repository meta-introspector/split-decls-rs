// Generated macro for impl_162 (impl)
macro_rules! Depcrate_extra_checksimpl_162 {
() => {
// Module: crate::extra_checks
// Provides: {"impl_162"}
// Dependencies: {}
impl FromStr for ExtraCheckLang { type Err = ExtraCheckParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "py" => Self :: Py , "shell" => Self :: Shell , "cpp" => Self :: Cpp , "spellcheck" => Self :: Spellcheck , "js" => Self :: Js , _ => return Err (ExtraCheckParseError :: UnknownLang (s . to_string ())) , }) } }
};
}
