use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
struct ServiceContext(*const c_void);
