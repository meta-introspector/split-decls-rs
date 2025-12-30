// Generated macro for normalization_provider (macro)
macro_rules! Depcrate_normalizernormalization_provider {
() => {
// Module: crate::normalizer
// Provides: {"normalization_provider"}
// Dependencies: {}
macro_rules ! normalization_provider { ($ marker : ident , $ serde_struct : ident , $ file_name : literal , $ conversion : expr , $ toml_data : ident) => { use icu :: normalizer :: provider ::$ marker ; impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; let $ toml_data : & normalizer_serde ::$ serde_struct = self . icuexport () ?. read_and_parse_toml (& format ! ("norm/{}/{}.toml" , if $ file_name == "nfd" || $ file_name == "nfkd" { TrieType :: Fast } else { self . trie_type () } , $ file_name)) ?; $ conversion } } impl crate :: IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { Ok (HashSet :: from_iter ([Default :: default ()])) } } } ; }
};
}
