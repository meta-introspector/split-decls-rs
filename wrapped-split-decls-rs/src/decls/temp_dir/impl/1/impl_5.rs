use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AsRef < Path > for MaybeTempDir { fn as_ref (& self) -> & Path { self . dir . path () } }