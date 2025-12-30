// Generated macro for implement_override (macro)
macro_rules! Depcrate_segmenterimplement_override {
() => {
// Module: crate::segmenter
// Provides: {"implement_override"}
// Dependencies: {}
macro_rules ! implement_override { ($ marker : ident , $ rules : literal , [$ ($ supported : expr) ,*]) => { impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse <$ marker >, DataError > { # [cfg (not (any (feature = "use_wasm" , feature = "use_icu4c")))] return Err (DataError :: custom ("icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules" ,) . with_req ($ marker :: INFO , req)) ; # [cfg (any (feature = "use_wasm" , feature = "use_icu4c"))] return { self . check_req ::<$ marker > (req) ?; let data = generate_rule_break_data_override (& hardcoded_segmenter_provider () , $ rules , self . trie_type () ,) ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data) , }) } ; } } impl crate :: IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { const SUPPORTED : & [& str] = & [$ ($ supported) ,*] ; Ok (SUPPORTED . iter () . map (| l | DataIdentifierCow :: from_locale (DataLocale :: try_from_str (l) . unwrap ())) . collect ()) } } } }
};
}
