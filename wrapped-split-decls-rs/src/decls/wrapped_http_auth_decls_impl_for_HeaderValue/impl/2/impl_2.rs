use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "http10")] impl HeaderValue for http10 :: HeaderValue { fn to_str (& self) -> Result < & str , ToStrError > { self . to_str () . map_err (| _ | ToStrError { _priv : () }) } }
}