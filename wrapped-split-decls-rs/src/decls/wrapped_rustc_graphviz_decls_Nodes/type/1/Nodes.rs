use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type Nodes < 'a , N > = Cow < 'a , [N] > ;
}