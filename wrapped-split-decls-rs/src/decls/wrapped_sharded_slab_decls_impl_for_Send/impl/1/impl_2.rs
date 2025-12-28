use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < T , C > Send for OwnedEntry < T , C > where T : Sync , C : cfg :: Config , { }
}