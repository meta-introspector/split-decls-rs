// Generated macro for RoundingModes (struct)
macro_rules! Depcrate_cldr_serde_currencies_supplementalRoundingModes {
() => {
// Module: crate::cldr_serde::currencies::supplemental
// Provides: {"RoundingModes"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct RoundingModes { # [serde (rename = "_rounding")] pub (crate) rounding : Option < String > , # [serde (rename = "_digits")] pub (crate) digits : Option < String > , # [serde (rename = "_cashRounding")] pub (crate) cash_rounding : Option < String > , # [serde (rename = "_cashDigits")] pub (crate) cash_digits : Option < String > , }
};
}
