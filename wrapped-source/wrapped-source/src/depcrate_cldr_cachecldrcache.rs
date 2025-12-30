// Generated macro for CldrCache (struct)
macro_rules! Depcrate_cldr_cacheCldrCache {
() => {
// Module: crate::cldr_cache
// Provides: {"CldrCache"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct CldrCache { pub (crate) serde_cache : SerdeCache , dir_suffix : OnceLock < Result < & 'static str , DataError > > , extended_locale_expander : OnceLock < Result < LocaleExpander , DataError > > , # [expect (clippy :: type_complexity)] pub (crate) calendar_eras : OnceLock < Result < BTreeMap < DatagenCalendar , Vec < (usize , EraData) > > , DataError > > , # [cfg (feature = "experimental")] pub (crate) transforms : OnceLock < Result < std :: sync :: Mutex < icu :: experimental :: transliterate :: RuleCollection > , DataError > , > , pub (crate) tz_caches : crate :: time_zones :: Caches , }
};
}
