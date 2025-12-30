// Generated macro for LocaleRules (struct)
macro_rules! Depcrate_cldr_serde_parent_localesLocaleRules {
() => {
// Module: crate::cldr_serde::parent_locales
// Provides: {"LocaleRules"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize , PartialOrd , Ord , Eq)] pub (crate) struct LocaleRules { # [serde (rename = "parentLocale")] pub (crate) parent_locale : Option < LocaleRule > , pub (crate) collations : Option < LocaleRule > , }
};
}
