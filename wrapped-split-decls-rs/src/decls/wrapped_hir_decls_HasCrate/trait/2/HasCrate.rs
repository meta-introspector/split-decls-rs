use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Trait for obtaining the defining crate of an item."] pub trait HasCrate { fn krate (& self , db : & dyn HirDatabase) -> Crate ; }
}