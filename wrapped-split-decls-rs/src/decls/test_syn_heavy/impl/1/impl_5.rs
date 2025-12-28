use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FunctionVisitor { fn new () -> Self { Self { call_count : 0 } } }