use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Fields { message : field :: Field , target : field :: Field , module : field :: Field , file : field :: Field , line : field :: Field , }
}