// Generated macro for normalization_non_recursive_decomposition_supplement_provider (macro)
macro_rules! Depcrate_normalizernormalization_non_recursive_decomposition_supplement_provider {
() => {
// Module: crate::normalizer
// Provides: {"normalization_non_recursive_decomposition_supplement_provider"}
// Dependencies: {}
macro_rules ! normalization_non_recursive_decomposition_supplement_provider { ($ marker : ident , $ file_name : literal) => { normalization_provider ! ($ marker , NonRecursiveDecompositionSupplement , $ file_name , { let trie = CodePointTrie ::< u32 >:: try_from (& toml_data . trie) . map_err (| e | DataError :: custom ("trie conversion") . with_display_context (& e)) ?; let scalars24 = toml_data . scalars32 . iter () . map (|& u | { u . try_into () . map_err (| _ | DataError :: custom ("scalars24 conversion")) }) . collect ::< Result < Vec < char >, DataError >> () ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (NonRecursiveDecompositionSupplement { trie , scalars24 : ZeroVec :: alloc_from_slice (& scalars24) , }) , }) } , toml_data) ; } ; }
};
}
