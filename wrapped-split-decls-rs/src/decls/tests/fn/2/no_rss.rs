use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn no_rss () { let entry = JsonTimePassesEntry { pass : "typeck" , time : 56.1 , start_rss : None , end_rss : None } ; assert_eq ! (entry . to_string () , r#"{"pass":"typeck","time":56.1,"rss_start":null,"rss_end":null}"#) }
}