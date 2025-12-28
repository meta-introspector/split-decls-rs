use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , PartialEq , Eq , Hash)] pub struct SpanAnchor { pub file_id : EditionedFileId , pub ast_id : ErasedFileAstId , }