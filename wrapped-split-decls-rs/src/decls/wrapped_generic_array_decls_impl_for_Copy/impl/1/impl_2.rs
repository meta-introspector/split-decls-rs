use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Copy , U : Copy > Copy for GenericArrayImplOdd < T , U > { }
}