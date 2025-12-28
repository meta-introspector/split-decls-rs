use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct RegistryData { thread_limit : NonZero < usize > , threads : Mutex < usize > , }
}