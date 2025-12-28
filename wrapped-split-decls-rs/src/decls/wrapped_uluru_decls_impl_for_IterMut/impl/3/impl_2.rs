use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T , const N : usize > IterMut < 'a , T , N > { fn next (& mut self) -> Option < (u16 , & mut T) > { let index = self . pos ; let entry = self . cache . entries . get_mut (index as usize) ? ; self . pos = if index == self . cache . tail { N as u16 } else { entry . next } ; Some ((index , & mut entry . val)) } }
}