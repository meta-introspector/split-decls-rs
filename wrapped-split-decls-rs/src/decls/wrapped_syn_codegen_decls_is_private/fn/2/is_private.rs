use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cfg (feature = "serde")] fn is_private (data : & Data) -> bool { match data { Data :: Private => true , Data :: Struct (_) | Data :: Enum (_) => false , } }
}