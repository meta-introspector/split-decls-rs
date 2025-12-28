use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < L , R , Target > AsRef < [Target] > for Either < L , R > where L : AsRef < [Target] > , R : AsRef < [Target] > , { fn as_ref (& self) -> & [Target] { for_both ! (self , inner => inner . as_ref ()) } }
}