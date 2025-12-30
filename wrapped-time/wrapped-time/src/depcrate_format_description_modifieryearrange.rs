// Generated macro for YearRange (enum)
macro_rules! Depcrate_format_description_modifierYearRange {
() => {
// Module: crate::format_description::modifier
// Provides: {"YearRange"}
// Dependencies: {}
# [doc = " The range of years that are supported."] # [doc = ""] # [doc = " This modifier has no effect when the year repr is [`LastTwo`](YearRepr::LastTwo)."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum YearRange { # [doc = " Years between -9999 and 9999 are supported."] Standard , # [doc = " Years between -999_999 and 999_999 are supported, with the sign being required if the year"] # [doc = " contains more than four digits."] # [doc = ""] # [doc = " If the `large-dates` feature is not enabled, this variant is equivalent to `Standard`."] Extended , }
};
}
