use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type TestGraph = LinkedGraph < & 'static str , & 'static str > ;