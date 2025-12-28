use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Entries { map : HashMap < String , Predefined > , }
}