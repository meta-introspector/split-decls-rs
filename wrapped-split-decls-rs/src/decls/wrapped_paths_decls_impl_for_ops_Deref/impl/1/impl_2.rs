use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ops :: Deref for RelPathBuf { type Target = RelPath ; fn deref (& self) -> & RelPath { self . as_path () } }
}