use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type Trivial = NodeLabels < & 'static str > ;
}