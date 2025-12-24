use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
struct Runner {
    tests: Vec<Test>,
}
