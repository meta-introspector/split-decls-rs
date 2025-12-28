use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > Display for Action < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Action :: Get (_) => f . write_str ("get") , Action :: Login (_) => f . write_str ("login") , Action :: Logout => f . write_str ("logout") , Action :: Unknown => f . write_str ("<unknown>") , } } }
}