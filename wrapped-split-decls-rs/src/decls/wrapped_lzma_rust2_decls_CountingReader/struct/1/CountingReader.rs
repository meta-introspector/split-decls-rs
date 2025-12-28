use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct CountingReader < R > { inner : R , bytes_read : u64 , }
}