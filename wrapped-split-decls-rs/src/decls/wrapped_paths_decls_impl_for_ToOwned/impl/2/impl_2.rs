use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ToOwned for AbsPath { type Owned = AbsPathBuf ; fn to_owned (& self) -> Self :: Owned { AbsPathBuf (self . 0 . to_owned ()) } }