use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type ExpandResult < T > = ValueResult < T , ExpandError > ;
}