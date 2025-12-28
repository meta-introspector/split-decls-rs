use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl MacCall { pub fn span (& self) -> Span { self . path . span . to (self . args . dspan . entire ()) } }