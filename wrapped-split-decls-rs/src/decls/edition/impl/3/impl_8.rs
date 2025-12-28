use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for Edition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = match * self { Edition :: Edition2015 => "2015" , Edition :: Edition2018 => "2018" , Edition :: Edition2021 => "2021" , Edition :: Edition2024 => "2024" , Edition :: EditionFuture => "future" , } ; write ! (f , "{s}") } }
}