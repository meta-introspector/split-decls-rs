use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] struct RawIter { yielded : usize , bucket : usize , bucket_size : usize , index : usize , }