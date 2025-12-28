use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc (hidden)] pub trait AnyExpectations : Any + Send + Sync { }
}