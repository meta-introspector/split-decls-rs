use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub (super) type FieldMap = HashMap < String , TokenStream > ;
}