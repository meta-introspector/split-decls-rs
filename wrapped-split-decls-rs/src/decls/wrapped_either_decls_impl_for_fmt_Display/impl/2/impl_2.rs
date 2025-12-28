use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < L , R > fmt :: Display for Either < L , R > where L : fmt :: Display , R : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for_both ! (self , inner => inner . fmt (f)) } }
}