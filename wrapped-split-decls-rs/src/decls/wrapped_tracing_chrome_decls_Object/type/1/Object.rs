use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type Object = serde_json :: Map < String , JsonValue > ;
}