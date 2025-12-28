use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RenderedExpandError { pub message : String , pub error : bool , pub kind : & 'static str , }