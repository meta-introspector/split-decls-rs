use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : RefCnt , S : Strategy < T > > Deref for Guard < T , S > { type Target = T ; # [inline] fn deref (& self) -> & T { self . inner . borrow () } }
}