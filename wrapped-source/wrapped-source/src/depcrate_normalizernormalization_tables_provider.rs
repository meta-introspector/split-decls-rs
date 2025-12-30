// Generated macro for normalization_tables_provider (macro)
macro_rules! Depcrate_normalizernormalization_tables_provider {
() => {
// Module: crate::normalizer
// Provides: {"normalization_tables_provider"}
// Dependencies: {}
macro_rules ! normalization_tables_provider { ($ marker : ident , $ file_name : literal) => { normalization_provider ! ($ marker , DecompositionTables , $ file_name , { let scalars24 = toml_data . scalars32 . iter () . map (|& u | { u . try_into () . map_err (| _ | DataError :: custom ("scalars24 conversion")) }) . collect ::< Result < Vec < char >, DataError >> () ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (DecompositionTables { scalars16 : ZeroVec :: alloc_from_slice (& toml_data . scalars16) , scalars24 : ZeroVec :: alloc_from_slice (& scalars24) , }) , }) } , toml_data) ; } ; }
};
}
