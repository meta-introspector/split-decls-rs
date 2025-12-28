use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Deref for Utf8PathBuf { type Target = Utf8Path ; fn deref (& self) -> & Utf8Path { self . as_path () } }