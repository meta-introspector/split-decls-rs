use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , C > OwnedEntry < T , C > where C : cfg :: Config , { # [doc = " Returns the key used to access this guard"] pub fn key (& self) -> usize { self . key } # [inline (always)] fn value (& self) -> & T { unsafe { self . value . as_ref () } } }
}