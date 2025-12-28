use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Clone for SendPtr < T > { fn clone (& self) -> Self { * self } }
}