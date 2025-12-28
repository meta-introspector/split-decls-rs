use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FromStr for Utf8PathBuf { type Err = Infallible ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (Utf8PathBuf (s . into ())) } }