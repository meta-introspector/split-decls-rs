// Generated macro for SubsecondDigits (enum)
macro_rules! Depcrate_format_description_modifierSubsecondDigits {
() => {
// Module: crate::format_description::modifier
// Provides: {"SubsecondDigits"}
// Dependencies: {}
# [doc = " The number of digits present in a subsecond representation."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum SubsecondDigits { # [doc = " Exactly one digit."] One , # [doc = " Exactly two digits."] Two , # [doc = " Exactly three digits."] Three , # [doc = " Exactly four digits."] Four , # [doc = " Exactly five digits."] Five , # [doc = " Exactly six digits."] Six , # [doc = " Exactly seven digits."] Seven , # [doc = " Exactly eight digits."] Eight , # [doc = " Exactly nine digits."] Nine , # [doc = " Any number of digits (up to nine) that is at least one. When formatting, the minimum digits"] # [doc = " necessary will be used."] OneOrMore , }
};
}
