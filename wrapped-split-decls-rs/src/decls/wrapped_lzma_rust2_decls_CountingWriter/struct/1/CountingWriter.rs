use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [cfg (feature = "encoder")] struct CountingWriter < W > { inner : W , bytes_written : u64 , }
}