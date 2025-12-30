// Generated macro for implement (macro)
macro_rules! Depcrate_segmenterimplement {
() => {
// Module: crate::segmenter
// Provides: {"implement"}
// Dependencies: {}
macro_rules ! implement { ($ marker : ident , $ rules : literal) => { impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { # [cfg (not (any (feature = "use_wasm" , feature = "use_icu4c")))] return Err (DataError :: custom ("icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules" ,) . with_req ($ marker :: INFO , req)) ; # [cfg (any (feature = "use_wasm" , feature = "use_icu4c"))] return { self . check_req ::<$ marker > (req) ?; let data = generate_rule_break_data (& hardcoded_segmenter_provider () , $ rules , self . trie_type () ,) ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data) , }) } ; } } impl crate :: IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { Ok (HashSet :: from_iter ([Default :: default ()])) } } } }
};
}
