use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type TestGraph = LinkedGraph < & 'static str , & 'static str > ;
}