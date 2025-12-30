// Generated macro for DecimalPattern (struct)
macro_rules! Depcrate_decimal_decimal_patternDecimalPattern {
() => {
// Module: crate::decimal::decimal_pattern
// Provides: {"DecimalPattern"}
// Dependencies: {}
# [doc = " Representation of a UTS-35 number pattern, including positive subpattern (required) and negative"] # [doc = " subpattern (optional)."] # [derive (Debug , PartialEq)] pub (crate) struct DecimalPattern { pub (crate) positive : DecimalSubPattern , pub (crate) negative : Option < DecimalSubPattern > , }
};
}
