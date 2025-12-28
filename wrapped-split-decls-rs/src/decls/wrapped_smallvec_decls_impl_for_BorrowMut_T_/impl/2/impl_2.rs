use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > BorrowMut < [T] > for SmallVec < T , N > { # [inline] fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
}