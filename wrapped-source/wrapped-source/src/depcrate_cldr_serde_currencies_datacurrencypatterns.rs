// Generated macro for CurrencyPatterns (struct)
macro_rules! Depcrate_cldr_serde_currencies_dataCurrencyPatterns {
() => {
// Module: crate::cldr_serde::currencies::data
// Provides: {"CurrencyPatterns"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct CurrencyPatterns { # [serde (rename = "symbol")] pub (crate) short : Option < String > , # [serde (rename = "symbol-alt-narrow")] pub (crate) narrow : Option < String > , # [serde (rename = "displayName")] pub (crate) display_name : Option < String > , # [serde (rename = "displayName-count-0")] pub (crate) explicit_zero : Option < String > , # [serde (rename = "displayName-count-1")] pub (crate) explicit_one : Option < String > , # [serde (rename = "displayName-count-zero")] pub (crate) zero : Option < String > , # [serde (rename = "displayName-count-one")] pub (crate) one : Option < String > , # [serde (rename = "displayName-count-two")] pub (crate) two : Option < String > , # [serde (rename = "displayName-count-few")] pub (crate) few : Option < String > , # [serde (rename = "displayName-count-many")] pub (crate) many : Option < String > , # [serde (rename = "displayName-count-other")] pub (crate) other : Option < String > , }
};
}
