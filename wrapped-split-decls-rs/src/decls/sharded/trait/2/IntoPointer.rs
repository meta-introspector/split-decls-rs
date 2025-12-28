use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait IntoPointer { # [doc = " Returns a pointer which outlives `self`."] fn into_pointer (& self) -> * const () ; }
}