use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static FIELD_NAMES : & [& str] = & ["message" , "log.target" , "log.module_path" , "log.file" , "log.line" ,] ;