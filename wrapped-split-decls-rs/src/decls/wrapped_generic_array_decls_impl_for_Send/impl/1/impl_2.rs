use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : Send , N : ArrayLength > Send for GenericArray < T , N > { }
}