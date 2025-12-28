use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct JsonTimePassesEntry < 'a > { pass : & 'a str , time : f64 , start_rss : Option < usize > , end_rss : Option < usize > , }