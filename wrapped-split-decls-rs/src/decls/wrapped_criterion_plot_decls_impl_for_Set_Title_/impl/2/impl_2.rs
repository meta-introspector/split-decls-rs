use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Set < Title > for Figure { # [doc = " Sets the title"] fn set (& mut self , title : Title) -> & mut Figure { self . title = Some (title . 0) ; self } }
}