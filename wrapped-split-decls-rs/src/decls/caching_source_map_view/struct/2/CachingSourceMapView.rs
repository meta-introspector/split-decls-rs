use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone)] pub struct CachingSourceMapView < 'sm > { source_map : & 'sm SourceMap , line_cache : [CacheEntry ; 3] , time_stamp : usize , }
}