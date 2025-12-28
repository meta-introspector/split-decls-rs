use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct JsonVisitor < 'a > { object : & 'a mut Object , }
}