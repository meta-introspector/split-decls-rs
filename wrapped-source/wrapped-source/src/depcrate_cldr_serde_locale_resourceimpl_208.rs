// Generated macro for impl_208 (impl)
macro_rules! Depcrate_cldr_serde_locale_resourceimpl_208 {
() => {
// Module: crate::cldr_serde::locale_resource
// Provides: {"impl_208"}
// Dependencies: {}
impl < 'de , T > Visitor < 'de > for SingleLocaleMapVisitor < T > where T : Deserialize < 'de > , { type Value = SingleLocaleMap < T > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a value keyed by a locale") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let Some (locale) = access . next_key :: < LanguageIdentifier > () ? else { return Err (M :: Error :: missing_field ("<LOCALE>")) ; } ; let value = access . next_value :: < T > () ? ; if access . next_key :: < LanguageIdentifier > () ? . is_some () { return Err (M :: Error :: duplicate_field ("<LOCALE>")) ; } Ok (SingleLocaleMap { _locale : locale , value , }) } }
};
}
