// Generated macro for impl_69 (impl)
macro_rules! Depcrate_extracted_derived_joining_typeimpl_69 {
() => {
// Module: crate::extracted::derived_joining_type
// Provides: {"impl_69"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedJoiningType { type Err = Error ; fn from_str (line : & str) -> Result < DerivedJoiningType , Error > { let (codepoints , joining_type) = parse_codepoint_association (line) ? ; Ok (DerivedJoiningType { codepoints , joining_type : joining_type . to_string () , }) } }
};
}
