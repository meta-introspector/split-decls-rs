use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type HammingResult = Result < usize , StrSimError > ;
}