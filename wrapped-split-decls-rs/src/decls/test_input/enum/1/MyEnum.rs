use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum MyEnum { Variant1 , Variant2 (i32) , Variant3 { x : i32 , y : i32 } , }
}