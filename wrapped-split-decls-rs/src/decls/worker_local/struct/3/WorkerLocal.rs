use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Holds worker local values for each possible thread in a registry. You can only access the"] # [doc = " worker local value through the `Deref` impl on the registry associated with the thread it was"] # [doc = " created on. It will panic otherwise."] pub struct WorkerLocal < T > { locals : Box < [CacheAligned < T >] > , registry : Registry , }
}