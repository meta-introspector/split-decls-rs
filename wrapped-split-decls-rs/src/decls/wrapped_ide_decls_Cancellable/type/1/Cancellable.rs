use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type Cancellable < T > = Result < T , Cancelled > ;
}