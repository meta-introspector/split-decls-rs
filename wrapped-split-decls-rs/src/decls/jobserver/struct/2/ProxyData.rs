use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ProxyData { # [doc = " The number of tokens assigned to threads."] # [doc = " If this is 0, a single token is still assigned to this process, but is unused."] used : u16 , # [doc = " The number of threads requesting a token"] pending : u16 , }
}