use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > TryFrom < & 'a Utf8Path > for & 'a AbsPath { type Error = & 'a Utf8Path ; fn try_from (path : & 'a Utf8Path) -> Result < & 'a AbsPath , & 'a Utf8Path > { if ! path . is_absolute () { return Err (path) ; } Ok (AbsPath :: assert (path)) } }