// Generated macro for impl_89 (impl)
macro_rules! Depcrate_parseimpl_89 {
() => {
// Module: crate::parse
// Provides: {"impl_89"}
// Dependencies: {}
impl FromStr for Prerelease { type Err = Error ; fn from_str (text : & str) -> Result < Self , Self :: Err > { let (pre , rest) = prerelease_identifier (text) ? ; if ! rest . is_empty () { return Err (Error :: new (ErrorKind :: IllegalCharacter (Position :: Pre))) ; } Ok (pre) } }
};
}
