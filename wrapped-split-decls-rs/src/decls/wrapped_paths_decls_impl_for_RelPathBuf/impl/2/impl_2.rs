use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl RelPathBuf { # [doc = " Coerces to a `RelPath` slice."] # [doc = ""] # [doc = " Equivalent of [`Utf8PathBuf::as_path`] for `RelPathBuf`."] pub fn as_path (& self) -> & RelPath { RelPath :: new_unchecked (self . 0 . as_path ()) } }