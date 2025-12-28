use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > crate :: sealed :: Sealed < T > for tracing :: Span { }