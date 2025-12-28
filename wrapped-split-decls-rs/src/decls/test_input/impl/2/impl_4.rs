use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MyTrait for MyStruct { fn method1 (& self) -> i32 { self . field1 } fn method2 (& mut self , value : i32) { self . field1 = value ; } }
}