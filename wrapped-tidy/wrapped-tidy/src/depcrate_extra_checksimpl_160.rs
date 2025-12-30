// Generated macro for impl_160 (impl)
macro_rules! Depcrate_extra_checksimpl_160 {
() => {
// Module: crate::extra_checks
// Provides: {"impl_160"}
// Dependencies: {}
impl FromStr for ExtraCheckArg { type Err = ExtraCheckParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut auto = false ; let mut parts = s . split (':') ; let Some (mut first) = parts . next () else { return Err (ExtraCheckParseError :: Empty) ; } ; if first == "auto" { let Some (part) = parts . next () else { return Err (ExtraCheckParseError :: AutoRequiresLang) ; } ; auto = true ; first = part ; } let second = parts . next () ; if parts . next () . is_some () { return Err (ExtraCheckParseError :: TooManyParts) ; } let arg = Self { auto , lang : first . parse () ? , kind : second . map (| s | s . parse ()) . transpose () ? } ; if ! arg . has_supported_kind () { return Err (ExtraCheckParseError :: UnsupportedKindForLang) ; } Ok (arg) } }
};
}
