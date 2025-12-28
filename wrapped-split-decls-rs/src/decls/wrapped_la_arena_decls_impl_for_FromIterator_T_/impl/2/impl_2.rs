use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > FromIterator < T > for Arena < T > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { Arena { data : Vec :: from_iter (iter) , } } }
}