use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Borrow < AbsPath > for AbsPathBuf { fn borrow (& self) -> & AbsPath { self . as_path () } }