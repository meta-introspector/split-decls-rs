use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: ops :: Deref for BaseNString { type Target = str ; fn deref (& self) -> & str { self . buf [self . start ..] . as_str () } }