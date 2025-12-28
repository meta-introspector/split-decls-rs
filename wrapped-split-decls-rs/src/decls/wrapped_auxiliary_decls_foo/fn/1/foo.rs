use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn foo () { println ! ("x") ; let mut map = HashMap :: new () ; map . insert (1 , "foo") ; }
}