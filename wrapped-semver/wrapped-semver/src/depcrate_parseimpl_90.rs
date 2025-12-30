// Generated macro for impl_90 (impl)
macro_rules! Depcrate_parseimpl_90 {
() => {
// Module: crate::parse
// Provides: {"impl_90"}
// Dependencies: {}
impl FromStr for BuildMetadata { type Err = Error ; fn from_str (text : & str) -> Result < Self , Self :: Err > { let (build , rest) = build_identifier (text) ? ; if ! rest . is_empty () { return Err (Error :: new (ErrorKind :: IllegalCharacter (Position :: Build))) ; } Ok (build) } }
};
}
