// Generated macro for SourceDataProvider (struct)
macro_rules! DepcrateSourceDataProvider {
() => {
// Module: crate
// Provides: {"SourceDataProvider"}
// Dependencies: {}
# [doc = " An [`ExportableProvider`](icu_provider::export::ExportableProvider) backed by raw CLDR and ICU data."] # [doc = ""] # [doc = " This provider covers all markers that are used by ICU4X. It is intended as the canonical"] # [doc = " provider for `ExportDriver::export`."] # [doc = ""] # [doc = " If a required data source has not been set, `DataProvider::load` will"] # [doc = " fail with the appropriate error:"] # [doc = " * [`is_missing_cldr_error`](Self::is_missing_cldr_error)"] # [doc = " * [`is_missing_icuexport_error`](Self::is_missing_icuexport_error)"] # [doc = " * [`is_missing_segmenter_lstm_error`](Self::is_missing_segmenter_lstm_error)"] # [allow (clippy :: exhaustive_structs)] # [derive (Debug , Clone)] pub struct SourceDataProvider { cldr_paths : Option < Arc < CldrCache > > , icuexport_paths : Option < Arc < SerdeCache > > , segmenter_lstm_paths : Option < Arc < SerdeCache > > , tzdb_paths : Option < Arc < TzdbCache > > , trie_type : TrieType , collation_root_han : CollationRootHan , pub (crate) timezone_horizon : time_zones :: Timestamp , # [expect (clippy :: type_complexity)] requests_cache : Arc < FrozenMap < DataMarkerInfo , Box < OnceLock < Result < HashSet < DataIdentifierCow < 'static > > , DataError > > > , > , > , }
};
}
