use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Set < Size > for Figure { # [doc = " Changes the figure size"] fn set (& mut self , size : Size) -> & mut Figure { self . size = Some ((size . 0 , size . 1)) ; self } }
}