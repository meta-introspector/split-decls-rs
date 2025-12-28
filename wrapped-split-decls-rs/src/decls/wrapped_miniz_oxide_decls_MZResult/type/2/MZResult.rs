use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " `Result` alias for all miniz status codes both successful and failed."] pub type MZResult = Result < MZStatus , MZError > ;
}