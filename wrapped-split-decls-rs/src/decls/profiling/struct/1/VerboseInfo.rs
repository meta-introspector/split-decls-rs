use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct VerboseInfo { start_time : Instant , start_rss : Option < usize > , message : String , format : TimePassesFormat , }