// Generated macro for impl_164 (impl)
macro_rules! Depcrate_extra_checksimpl_164 {
() => {
// Module: crate::extra_checks
// Provides: {"impl_164"}
// Dependencies: {}
impl FromStr for ExtraCheckKind { type Err = ExtraCheckParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "lint" => Self :: Lint , "fmt" => Self :: Fmt , "typecheck" => Self :: Typecheck , _ => return Err (ExtraCheckParseError :: UnknownKind (s . to_string ())) , }) } }
};
}
