// Generated macro for ParsedPlaceholder (struct)
macro_rules! Depcrate_decimal_compact_decimal_patternParsedPlaceholder {
() => {
// Module: crate::decimal::compact_decimal_pattern
// Provides: {"ParsedPlaceholder"}
// Dependencies: {}
# [doc = " Represents the placeholder in a compact decimal pattern as its position in"] # [doc = " the associated text and the number of 0s (which, together with the type"] # [doc = " associated with the pattern, determines the power of ten being abbreviated)."] # [derive (PartialEq , Clone)] struct ParsedPlaceholder { # [doc = " The position in the literal text where the placeholder is to be inserted;"] # [doc = " in particular, this is 0 for insertion at the beginning, which is the"] # [doc = " most frequent case, as in \"00 mille\"."] pub (crate) index : usize , pub (crate) number_of_0s : i8 , }
};
}
