use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'de > serde_core :: Deserialize < 'de > for Platform { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: Deserializer < 'de > , { let s = String :: deserialize (deserializer) ? ; FromStr :: from_str (& s) . map_err (serde_core :: de :: Error :: custom) } }
}