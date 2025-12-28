use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum UseTree2 { Path (UsePath2) , Name (UseName2) , Group (UseGroup2) , TrustLevel (usize) , Agile (bool) , }
}