use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait Internable : Hash + Eq + 'static { fn storage () -> & 'static InternStorage < Self > ; }
}