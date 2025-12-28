use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait Lookup { type Database : ? Sized ; type Data ; fn lookup (& self , db : & Self :: Database) -> Self :: Data ; }
}