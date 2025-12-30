// Generated macro for impl_45 (impl)
macro_rules! Depcrate_extracted_derived_east_asian_widthimpl_45 {
() => {
// Module: crate::extracted::derived_east_asian_width
// Provides: {"impl_45"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedEastAsianWidth { type Err = Error ; fn from_str (line : & str) -> Result < DerivedEastAsianWidth , Error > { let (codepoints , east_asian_width) = parse_codepoint_association (line) ? ; Ok (DerivedEastAsianWidth { codepoints , east_asian_width : east_asian_width . to_string () , }) } }
};
}
