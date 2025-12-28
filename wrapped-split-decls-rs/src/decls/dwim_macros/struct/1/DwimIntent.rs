use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct DwimIntent { keywords : Vec < String > , context : String , }
}