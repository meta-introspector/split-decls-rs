use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , C : cfg :: Config > Entry < '_ , T , C > { # [doc = " Returns the key used to access the guard."] pub fn key (& self) -> usize { self . key } # [inline (always)] fn value (& self) -> & T { unsafe { self . value . as_ref () } } }
}