use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DaoPolicy { pub fn new () -> Self { Self { allowed_commands : vec ! ["ls" . to_string () , "cat" . to_string () , "echo" . to_string () ,] , approval_threshold : 0.66 , } } pub fn approve_command (& self , cmd : & str) -> bool { if self . allowed_commands . contains (& cmd . to_string ()) { return true ; } let dangerous_commands = ["rm" , "dd" , "format" , "del" , "sudo"] ; for dangerous in & dangerous_commands { if cmd . contains (dangerous) { return false ; } } false } }
}