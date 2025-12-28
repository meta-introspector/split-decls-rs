use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Tree < T : Send > { value : T , children : Vec < Tree < T > > , }
}