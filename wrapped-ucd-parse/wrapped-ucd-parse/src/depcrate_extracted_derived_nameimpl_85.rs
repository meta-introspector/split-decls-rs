// Generated macro for impl_85 (impl)
macro_rules! Depcrate_extracted_derived_nameimpl_85 {
() => {
// Module: crate::extracted::derived_name
// Provides: {"impl_85"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedName { type Err = Error ; fn from_str (line : & str) -> Result < DerivedName , Error > { let (codepoints , name) = parse_codepoint_association (line) ? ; Ok (DerivedName { codepoints , name : name . to_string () }) } }
};
}
