// Generated macro for PluralRulesPattern (struct)
macro_rules! Depcrate_cldr_serde_date_fieldsPluralRulesPattern {
() => {
// Module: crate::cldr_serde::date_fields
// Provides: {"PluralRulesPattern"}
// Dependencies: {}
# [derive (Debug , Deserialize , Default)] pub (crate) struct PluralRulesPattern { # [serde (rename = "relativeTimePattern-count-0")] pub (crate) explicit_zero : Option < PatternString < SinglePlaceholder > > , # [serde (rename = "relativeTimePattern-count-1")] pub (crate) explicit_one : Option < PatternString < SinglePlaceholder > > , # [serde (rename = "relativeTimePattern-count-zero")] pub (crate) zero : Option < PatternString < SinglePlaceholder > > , # [serde (rename = "relativeTimePattern-count-one")] pub (crate) one : Option < PatternString < SinglePlaceholder > > , # [serde (rename = "relativeTimePattern-count-two")] pub (crate) two : Option < PatternString < SinglePlaceholder > > , # [serde (rename = "relativeTimePattern-count-few")] pub (crate) few : Option < PatternString < SinglePlaceholder > > , # [serde (rename = "relativeTimePattern-count-many")] pub (crate) many : Option < PatternString < SinglePlaceholder > > , # [serde (rename = "relativeTimePattern-count-other")] pub (crate) other : PatternString < SinglePlaceholder > , }
};
}
