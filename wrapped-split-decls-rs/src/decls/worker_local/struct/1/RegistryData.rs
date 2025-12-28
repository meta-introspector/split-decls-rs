use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct RegistryData { thread_limit : NonZero < usize > , threads : Mutex < usize > , }