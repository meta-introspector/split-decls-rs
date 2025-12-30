// Generated macro for ParsedPattern (struct)
macro_rules! Depcrate_decimal_compact_decimal_patternParsedPattern {
() => {
// Module: crate::decimal::compact_decimal_pattern
// Provides: {"ParsedPattern"}
// Dependencies: {}
# [doc = " A [`ParsedPattern`] represents a compact decimal pattern, which consists of"] # [doc = " literal text with an optional placeholder.  The literal text is unescaped,"] # [doc = " and the information about the number of 0s in the placeholder is stored"] # [doc = " separately."] # [derive (PartialEq , Clone)] struct ParsedPattern { # [doc = " The unescaped literal text, e.g., \" mille\" for the pattern \"00 mille\","] # [doc = " \"mille\" for the pattern \"mille\"."] pub (crate) literal_text : Cow < 'static , str > , # [doc = " The placeholder; `None` for patterns such as \"mille\"."] pub (crate) placeholder : Option < ParsedPlaceholder > , }
};
}
