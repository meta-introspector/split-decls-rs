use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Used to construct a `SourceMap` with `SourceMap::with_inputs`."] pub struct SourceMapInputs { pub file_loader : Box < dyn FileLoader + Send + Sync > , pub path_mapping : FilePathMapping , pub hash_kind : SourceFileHashAlgorithm , pub checksum_hash_kind : Option < SourceFileHashAlgorithm > , }
}