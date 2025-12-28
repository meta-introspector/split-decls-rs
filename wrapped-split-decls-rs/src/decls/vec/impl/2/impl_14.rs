use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T > BorrowMut < IndexSlice < I , T > > for IndexVec < I , T > { fn borrow_mut (& mut self) -> & mut IndexSlice < I , T > { self } }
}