use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn with_rss () { let entry = JsonTimePassesEntry { pass : "typeck" , time : 56.1 , start_rss : Some (10) , end_rss : Some (20) } ; assert_eq ! (entry . to_string () , r#"{"pass":"typeck","time":56.1,"rss_start":10,"rss_end":20}"#) }