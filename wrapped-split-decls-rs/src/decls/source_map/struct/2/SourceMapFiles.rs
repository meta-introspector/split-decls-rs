use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Default)] struct SourceMapFiles { source_files : monotonic :: MonotonicVec < Arc < SourceFile > > , stable_id_to_source_file : UnhashMap < StableSourceFileId , Arc < SourceFile > > , }