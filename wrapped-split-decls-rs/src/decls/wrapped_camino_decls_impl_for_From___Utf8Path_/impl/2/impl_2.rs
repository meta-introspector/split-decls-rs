use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < & '_ Utf8Path > for Rc < Path > { fn from (path : & Utf8Path) -> Rc < Path > { AsRef :: < Path > :: as_ref (path) . into () } }