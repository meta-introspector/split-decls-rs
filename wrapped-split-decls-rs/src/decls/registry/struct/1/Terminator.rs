use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Terminator < 'a > (& 'a Arc < Registry >) ;
}