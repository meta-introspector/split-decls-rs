use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [cfg (feature = "utf8")] struct VtUtf8Receiver < 'a > (& 'a mut Option < char >) ;
}