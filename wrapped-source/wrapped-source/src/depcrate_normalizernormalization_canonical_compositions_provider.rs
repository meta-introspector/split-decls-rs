// Generated macro for normalization_canonical_compositions_provider (macro)
macro_rules! Depcrate_normalizernormalization_canonical_compositions_provider {
() => {
// Module: crate::normalizer
// Provides: {"normalization_canonical_compositions_provider"}
// Dependencies: {}
macro_rules ! normalization_canonical_compositions_provider { ($ marker : ident , $ file_name : literal) => { normalization_provider ! ($ marker , CanonicalCompositions , $ file_name , { Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (CanonicalCompositions { canonical_compositions : Char16Trie :: new (ZeroVec :: alloc_from_slice (& toml_data . compositions ,)) , }) , }) } , toml_data) ; } ; }
};
}
