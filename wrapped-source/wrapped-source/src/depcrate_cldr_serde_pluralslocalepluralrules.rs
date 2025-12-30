// Generated macro for LocalePluralRules (struct)
macro_rules! Depcrate_cldr_serde_pluralsLocalePluralRules {
() => {
// Module: crate::cldr_serde::plurals
// Provides: {"LocalePluralRules"}
// Dependencies: {}
# [derive (PartialEq , PartialOrd , Ord , Eq , Debug , Deserialize)] pub (crate) struct LocalePluralRules { # [serde (rename = "pluralRule-count-zero")] pub (crate) zero : Option < String > , # [serde (rename = "pluralRule-count-one")] pub (crate) one : Option < String > , # [serde (rename = "pluralRule-count-two")] pub (crate) two : Option < String > , # [serde (rename = "pluralRule-count-few")] pub (crate) few : Option < String > , # [serde (rename = "pluralRule-count-many")] pub (crate) many : Option < String > , }
};
}
