use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait Intern { type Database : ? Sized ; type ID ; fn intern (self , db : & Self :: Database) -> Self :: ID ; }
}