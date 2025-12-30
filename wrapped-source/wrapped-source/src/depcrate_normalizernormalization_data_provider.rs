// Generated macro for normalization_data_provider (macro)
macro_rules! Depcrate_normalizernormalization_data_provider {
() => {
// Module: crate::normalizer
// Provides: {"normalization_data_provider"}
// Dependencies: {}
macro_rules ! normalization_data_provider { ($ marker : ident , $ file_name : literal) => { normalization_provider ! ($ marker , DecompositionData , $ file_name , { let trie = CodePointTrie ::< u32 >:: try_from (& toml_data . trie) . map_err (| e | DataError :: custom ("trie conversion") . with_display_context (& e)) ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (DecompositionData { trie , passthrough_cap : toml_data . cap , }) , }) } , toml_data) ; } ; }
};
}
