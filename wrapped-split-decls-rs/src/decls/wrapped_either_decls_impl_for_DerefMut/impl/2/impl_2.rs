use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < L , R > DerefMut for Either < L , R > where L : DerefMut , R : DerefMut < Target = L :: Target > , { fn deref_mut (& mut self) -> & mut Self :: Target { for_both ! (self , inner => & mut * inner) } }
}