use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " safe because `FileId` is a newtype of `u32`"] impl nohash_hasher :: IsEnabled for FileId { }