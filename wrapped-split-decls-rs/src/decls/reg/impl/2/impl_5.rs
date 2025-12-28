use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Reg { reg_ctor ! (i8 , Integer , 8) ; reg_ctor ! (i16 , Integer , 16) ; reg_ctor ! (i32 , Integer , 32) ; reg_ctor ! (i64 , Integer , 64) ; reg_ctor ! (i128 , Integer , 128) ; reg_ctor ! (f32 , Float , 32) ; reg_ctor ! (f64 , Float , 64) ; }
}