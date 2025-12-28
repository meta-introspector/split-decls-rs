use serde::{Deserialize, Serialize};
use std::collections::HashMap;

trait MyTrait { fn method1 (& self) -> i32 ; fn method2 (& mut self , value : i32) ; }