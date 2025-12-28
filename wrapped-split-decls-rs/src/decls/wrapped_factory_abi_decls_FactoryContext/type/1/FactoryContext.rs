use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FactoryContext = * mut c_void ;
}