use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < E > Edge < E > { pub fn source (& self) -> NodeIndex { self . source } pub fn target (& self) -> NodeIndex { self . target } pub fn source_or_target (& self , direction : Direction) -> NodeIndex { if direction == OUTGOING { self . target } else { self . source } } }