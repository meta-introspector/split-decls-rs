use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cfg (not (feature = "use_logging"))] fn env_logger_init () { }
}