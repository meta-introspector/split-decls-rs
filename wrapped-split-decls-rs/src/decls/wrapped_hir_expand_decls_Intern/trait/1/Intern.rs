use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Intern { type Database : ? Sized ; type ID ; fn intern (self , db : & Self :: Database) -> Self :: ID ; }