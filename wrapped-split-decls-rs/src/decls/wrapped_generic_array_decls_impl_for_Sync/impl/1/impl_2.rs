use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T : Sync , N : ArrayLength > Sync for GenericArray < T , N > { }
}