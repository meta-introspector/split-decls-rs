use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct DepthFirstTraversal < 'g , N , E > { graph : & 'g LinkedGraph < N , E > , stack : Vec < NodeIndex > , visited : DenseBitSet < usize > , direction : Direction , }
}