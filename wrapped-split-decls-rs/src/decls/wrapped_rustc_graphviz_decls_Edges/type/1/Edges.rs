use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type Edges < 'a , E > = Cow < 'a , [E] > ;
}