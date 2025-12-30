// Generated macro for DecimalSubPattern (struct)
macro_rules! Depcrate_decimal_decimal_patternDecimalSubPattern {
() => {
// Module: crate::decimal::decimal_pattern
// Provides: {"DecimalSubPattern"}
// Dependencies: {}
# [doc = " Representation of a UTS-35 number subpattern (part of a number pattern between ';'s)."] # [derive (Debug , PartialEq)] pub (crate) struct DecimalSubPattern { pub (crate) prefix : String , pub (crate) suffix : String , pub (crate) primary_grouping : u8 , pub (crate) secondary_grouping : u8 , pub (crate) min_fraction_digits : u8 , pub (crate) max_fraction_digits : u8 , }
};
}
