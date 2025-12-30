// Generated macro for NumberingSystemData (struct)
macro_rules! Depcrate_cldr_serde_numbersNumberingSystemData {
() => {
// Module: crate::cldr_serde::numbers
// Provides: {"NumberingSystemData"}
// Dependencies: {}
# [derive (PartialEq , Debug , Default)] pub (crate) struct NumberingSystemData { # [doc = " Map from numbering system to symbols"] pub (crate) symbols : HashMap < String , Symbols > , # [doc = " Map from numbering system to decimal formats"] pub (crate) formats : HashMap < String , DecimalFormats > , # [doc = " Map from numbering system to patterns"] pub (crate) currency_patterns : HashMap < String , CurrencyFormattingPatterns > , # [doc = " Map from numbering system to percent patterns"] pub (crate) percent_patterns : HashMap < String , PercentFormattingPatterns > , }
};
}
