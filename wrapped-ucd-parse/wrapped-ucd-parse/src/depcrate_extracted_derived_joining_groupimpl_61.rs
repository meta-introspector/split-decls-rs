// Generated macro for impl_61 (impl)
macro_rules! Depcrate_extracted_derived_joining_groupimpl_61 {
() => {
// Module: crate::extracted::derived_joining_group
// Provides: {"impl_61"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedJoiningGroup { type Err = Error ; fn from_str (line : & str) -> Result < DerivedJoiningGroup , Error > { let (codepoints , joining_group) = parse_codepoint_association (line) ? ; Ok (DerivedJoiningGroup { codepoints , joining_group : joining_group . to_string () , }) } }
};
}
