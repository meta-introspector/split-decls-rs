use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'g , N : Debug , E : Debug > ExactSizeIterator for DepthFirstTraversal < 'g , N , E > { }
}