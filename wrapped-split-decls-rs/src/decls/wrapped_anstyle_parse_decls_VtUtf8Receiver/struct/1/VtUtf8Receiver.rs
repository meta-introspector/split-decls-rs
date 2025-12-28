use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "utf8")] struct VtUtf8Receiver < 'a > (& 'a mut Option < char >) ;