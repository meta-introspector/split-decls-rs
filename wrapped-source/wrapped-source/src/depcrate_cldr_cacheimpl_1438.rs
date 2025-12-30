// Generated macro for impl_1438 (impl)
macro_rules! Depcrate_cldr_cacheimpl_1438 {
() => {
// Module: crate::cldr_cache
// Provides: {"impl_1438"}
// Dependencies: {}
impl < 'a > CldrDirNoLang < 'a > { pub (crate) fn read_and_parse < S > (& self , file_name : & str) -> Result < & 'a S , DataError > where for < 'de > S : serde :: Deserialize < 'de > + 'static + Send + Sync , { self . 0 . serde_cache . read_and_parse_json (& format ! ("{}/{}" , self . 1 , file_name)) } }
};
}
