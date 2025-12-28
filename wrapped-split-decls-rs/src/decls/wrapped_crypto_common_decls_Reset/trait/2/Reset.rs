use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Resettable types."] pub trait Reset { # [doc = " Reset state to its initial value."] fn reset (& mut self) ; }
}