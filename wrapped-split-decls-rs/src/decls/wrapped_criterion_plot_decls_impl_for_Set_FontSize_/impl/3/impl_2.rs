use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Set < FontSize > for Figure { # [doc = " Changes the size of the font"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `size` is a non-positive value"] fn set (& mut self , size : FontSize) -> & mut Figure { let size = size . 0 ; assert ! (size >= 0.) ; self . font_size = Some (size) ; self } }
}