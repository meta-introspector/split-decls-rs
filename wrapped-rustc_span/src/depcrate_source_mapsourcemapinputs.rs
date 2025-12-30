// Generated macro for SourceMapInputs (struct)
macro_rules! Depcrate_source_mapSourceMapInputs {
() => {
// Module: crate::source_map
// Provides: {"SourceMapInputs"}
// Dependencies: {}
# [doc = " Used to construct a `SourceMap` with `SourceMap::with_inputs`."] pub struct SourceMapInputs { pub file_loader : Box < dyn FileLoader + Send + Sync > , pub path_mapping : FilePathMapping , pub hash_kind : SourceFileHashAlgorithm , pub checksum_hash_kind : Option < SourceFileHashAlgorithm > , }
};
}
