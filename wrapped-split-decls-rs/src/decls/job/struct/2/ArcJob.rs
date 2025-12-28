use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents a job stored in an `Arc` -- like `HeapJob`, but may"] # [doc = " be turned into multiple `JobRef`s and called multiple times."] pub (super) struct ArcJob < BODY > where BODY : Fn (JobRefId) + Send + Sync , { job : BODY , }
}