use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl rustc_index :: Idx for OwnerId { # [inline] fn new (idx : usize) -> Self { OwnerId { def_id : LocalDefId { local_def_index : DefIndex :: from_usize (idx) , } , } } # [inline] fn index (self) -> usize { self . def_id . local_def_index . as_usize () } }