use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait HasContainer { fn container (& self , db : & dyn HirDatabase) -> ItemContainer ; }
}