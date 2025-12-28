use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (crate) struct Data { start : Instant , kvs : Vec < (& 'static str , String) > , written : bool , }