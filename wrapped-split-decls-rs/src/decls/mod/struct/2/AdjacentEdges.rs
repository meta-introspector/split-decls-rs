use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct AdjacentEdges < 'g , N , E > { graph : & 'g LinkedGraph < N , E > , direction : Direction , next : EdgeIndex , }
}