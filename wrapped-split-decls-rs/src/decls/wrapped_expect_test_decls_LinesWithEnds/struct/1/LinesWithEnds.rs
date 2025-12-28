use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct LinesWithEnds < 'a > { text : & 'a str , }
}