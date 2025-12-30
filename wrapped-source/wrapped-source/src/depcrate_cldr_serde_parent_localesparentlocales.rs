// Generated macro for ParentLocales (struct)
macro_rules! Depcrate_cldr_serde_parent_localesParentLocales {
() => {
// Module: crate::cldr_serde::parent_locales
// Provides: {"ParentLocales"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct ParentLocales { # [serde (rename = "parentLocale")] pub (crate) parent_locale : HashMap < LanguageIdentifier , LanguageIdentifier > , pub (crate) collations : BTreeMap < String , LanguageIdentifier > , # [serde (rename = "_localeRules" , default = "rules_backport")] pub (crate) rules : LocaleRules , }
};
}
