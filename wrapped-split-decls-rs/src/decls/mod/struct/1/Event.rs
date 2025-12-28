use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Event < N > { node : N , becomes : NodeStatus , }
}